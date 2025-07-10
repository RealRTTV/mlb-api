mod attributes;
mod util;

use crate::attributes::parse::ParseAttribute;
use crate::attributes::request::{RequestAttribute, RequestType};
use crate::attributes::try_from::TryFromAttribute;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;
use indexmap::IndexMap;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, AttrStyle, Attribute, DeriveInput, LitStr, Meta, MetaList, Token, Type};

#[proc_macro_derive(HttpCache, attributes(request, parse, try_from))]
pub fn http_cache_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    http_cache_derive0(input).unwrap_or_else(|e| e.into_compile_error().into())
}

fn parse_attributes(attrs: Vec<Attribute>) -> syn::Result<(TryFromAttribute, ParseAttribute, RequestAttribute)> {
    let mut try_from_attribute = None;
    let mut parse_attr = None;
    let mut request_attr = None;

    for attr in attrs {
        if let Attribute {
            style: AttrStyle::Outer,
            meta: Meta::List(MetaList { path, tokens, .. }),
            ..
        } = attr && let Some(ident) = path.get_ident() {
            let str = ident.to_string();
            let old = match &*str {
                "try_from" => try_from_attribute.replace(syn::parse2(tokens)?).map(|_| ()),
                "parse" => parse_attr.replace(syn::parse2(tokens)?).map(|_| ()),
                "request" => request_attr.replace(syn::parse2(tokens)?).map(|_| ()),
                _ => continue,
            };

            if old.is_some() {
                return Err(syn::Error::new(ident.span(), format!("Duplicate key found: {str}")))
            }
        }
    }

    Ok((
        try_from_attribute.ok_or_else(|| syn::Error::new(Span::call_site(), "Expected try_from attribute"))?,
        parse_attr.ok_or_else(|| syn::Error::new(Span::call_site(), "Expected parse attribute"))?,
        request_attr.ok_or_else(|| syn::Error::new(Span::call_site(), "Expected request attribute"))?,
    ))
}

fn http_cache_derive0(input: DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let struct_name = input.ident.clone();
    let (
        TryFromAttribute { lifetimes: try_from_lifetimes, r#type: try_from_type, field: ref try_from_field, destruct: try_from_destruct },
        ParseAttribute { r#macro: parse_macro, variant_name: ref parse_variant_name, r#type: parse_type },
        RequestAttribute { request_type, url: request_url }
    ) = parse_attributes(input.attrs)?;
    
    let json = match request_type {
        RequestType::GET => ureq::get(request_url).call().unwrap().into_body(),
        // RequestType::POST => ureq::post(request_url).send(()).unwrap().into_body(),
    }.read_json::<Value>().unwrap();

    let json = match json {
        Value::Array(values) => values,
        Value::Object(mut root) => {
            root.remove("copyright").expect("Expected copyright");
            if root.len() != 1 {
                panic!("Expected only one value besides copyright, found {}", root.len());
            }
            let Value::Array(inner) = root.into_iter().next().unwrap().1 else { panic!("Expected values to be an array") };
            inner
        }
        _ => panic!("Invalid json root, expected either array of entries, or object with copyright"),
    };

    let mut map: IndexMap<String, Vec<Value>> = IndexMap::new();

    for entry in json {
        let variant_name = &entry[parse_variant_name];
        if !variant_name.is_string() {
            panic!("Invalid variant name '{variant_name}' found");
        }
        let name = util::to_upper_snake_case(variant_name.as_str().expect("Expected variant name to be a string"));
        map.entry(name).or_insert(Vec::new()).push(entry);
    }

    let values = map.into_iter().flat_map(|(key, mut value)| {
        if value.len() == 1 {
            vec![(key, value.remove(0))]
        } else {
            value.into_iter().enumerate().map(|(idx, value)| (format!("{key}_{char}", char = (b'A' + idx as u8) as char), value)).collect::<Vec<_>>()
        }
    }).collect::<Vec<_>>();

    let variants = values.iter().map(|(name, entry)| {
        let name = syn::parse_str::<Ident>(&name).expect(&format!("Expected identifier '{name}' to be valid"));
        let tokens = entry.to_string().parse::<TokenStream>().unwrap();
        quote! {
            pub const #name: #parse_type = #parse_macro!(#tokens);
        }
    }).collect::<TokenStream>();
    
    let consts = quote! {
        impl #struct_name {
            #variants
        }
    };

    let branches = values.iter().map(|(variant_name, entry)| {
        let variant_name = format_ident!("{variant_name}");
        let inner = entry[try_from_field].to_string().parse::<TokenStream>().map_err(|e| syn::Error::new(Span::call_site(), format!("Failed to parse TryFrom match arm: {e}")))?;
        let arm = if try_from_destruct {
            quote! { #try_from_type(#inner) }
        } else {
            inner
        };
        Ok(quote! {
            #arm => #struct_name::#variant_name,
        })
    }).collect::<syn::Result<TokenStream>>()?;
    
    let try_from_impl = quote! {
        impl #parse_type {
            #[must_use]
            pub(crate) const fn __internal_const_try_from<#(#try_from_lifetimes),*>(value: #try_from_type) -> Option<Self> {
                Some(match value {
                    #branches
                    _ => return None,
                })
            }
        }
        
        impl<#(#try_from_lifetimes),*> ::core::convert::TryFrom<#try_from_type> for #parse_type {
            type Error = ();

            fn try_from(value: #try_from_type) -> Result<Self, Self::Error> {
                Ok(match value {
                    #branches
                    _ => return Err(()),
                })
            }
        }
    };

    Ok(quote! {
        #consts
        #try_from_impl
    }.into())
}

#[derive(Default)]
struct StaticCacheAttribute {
    url: Option<String>,
    request: Option<RequestType>,
    r#macro: Option<Ident>,
    name: Option<String>,
    from_str: Option<String>,
    const_type: Option<Type>,
}

impl Parse for StaticCacheAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        loop {
            if input.is_empty() {
                break;
            }

            {
                let key = Ident::parse_any(input)?.to_string();
                let _: Token![=] = input.parse()?;
                let old = match &*key {
                    "url" => this
                        .url
                        .replace(input.parse::<LitStr>()?.value().to_owned())
                        .map(|_| ()),
                    "request" => {
                        this.request.replace(input.parse::<LitStr>()?.value().parse::<RequestType>().expect(
                            "Expected a valid request such as `GET`",
                        )).map(|_| ())
                    },
                    "macro" => this.r#macro.replace(input.parse()?).map(|_| ()),
                    "name" => this.name.replace(input.parse::<LitStr>()?.value()).map(|_| ()),
                    "from_str" => this.from_str.replace(input.parse::<LitStr>()?.value()).map(|_| ()),
                    "const_type" => this.const_type.replace(input.parse::<Type>()?).map(|_| ()),
                    _ => panic!(
                        "Invalid key `{key}`. The list of valid options are: `format`, `url`, `macro`, `request`, `from_str` and `name`."
                    ),
                };
                if old.is_some() {
                    panic!("Duplicate key `{key}` found.");
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        Ok(this)
    }
}

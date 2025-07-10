use proc_macro2::Ident;
use syn::{bracketed, Lifetime, LitBool, LitStr, Token, Type};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseBuffer, ParseStream};

pub(crate) struct TryFromAttribute {
    pub(crate) lifetimes: Vec<Lifetime>,
    pub(crate) r#type: Type,
    pub(crate) field: String,
    pub(crate) destruct: bool,
}

impl Parse for TryFromAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let mut lifetimes = Vec::new();
        let mut r#type = None;
        let mut field = None;
        let mut destruct = false;

        loop {
            if input.is_empty() {
                break;
            }

            let key_span = input.span();
            let key: Ident = input.call(Ident::parse_any)?;
            let key = key.to_string();
            let _: Token![=] = input.parse()?;
            let old = match key.as_str() {
                "lifetimes" => {
                    let contents: ParseBuffer;
                    let _ = bracketed!(contents in input);
                    loop {
                        if contents.is_empty() {
                            break
                        }
                        
                        let lifetime = contents.parse()?;
                        lifetimes.push(lifetime);
                        
                        if contents.peek(Token![,]) {
                            contents.parse::<Token![,]>()?;
                        } else {
                            break
                        }
                    }
                    None
                }
                "type" => r#type.replace(input.parse::<Type>()?).map(|_| ()),
                "field" => field.replace(input.parse::<LitStr>()?.value()).map(|_| ()),
                "destruct" => {
                    destruct = input.parse::<LitBool>()?.value;
                    None
                },
                key => return Err(syn::Error::new(key_span, format!("Unknown key {key}"))),
            };
            
            if old.is_some() {
                return Err(syn::Error::new(key_span, format!("Duplicate key found: {key}")));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(Self {
            lifetimes,
            r#type: r#type.ok_or(syn::Error::new(span, "No 'type' entry found"))?,
            field: field.ok_or(syn::Error::new(span, "No 'field' entry found"))?,
            destruct,
        })
    }
}

use proc_macro2::Ident;
use syn::{LitStr, Path, Token, Type};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};

pub(crate) struct ParseAttribute {
    pub(crate) r#macro: Path,
    pub(crate) variant_name: String,
    pub(crate) r#type: Type,
}

impl Parse for ParseAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let mut r#macro = None;
        let mut variant_name = None;
        let mut r#type = None;

        loop {
            if input.is_empty() {
                break;
            }

            let key_span = input.span();
            let key: Ident = input.call(Ident::parse_any)?;
            let key = key.to_string();
            let _: Token![=] = input.parse()?;
            let old = match key.as_str() {
                "macro" => r#macro.replace(input.parse::<Path>()?).map(|_| ()),
                "variant_name" => variant_name.replace(input.parse::<LitStr>()?.value()).map(|_| ()),
                "type" => r#type.replace(input.parse::<Type>()?).map(|_| ()),
                key => return Err(syn::Error::new(key_span, format!("Unknown key: {key}"))),
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
            r#macro: r#macro.ok_or(syn::Error::new(span, "No 'macro' entry found"))?,
            variant_name: variant_name.ok_or(syn::Error::new(span, "No 'variant_name' entry found"))?,
            r#type: r#type.ok_or(syn::Error::new(span, "No 'type' entry found"))?,
        })
    }
}

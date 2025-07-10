use std::str::FromStr;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};
use syn::ext::IdentExt;

#[derive(Copy, Clone)]
pub(crate) enum RequestType {
    GET,
    // POST,
}

impl FromStr for RequestType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => Self::GET,
            // "POST" => Self::POST,
            _ => return Err(()),
        })
    }
}

pub(crate) struct RequestAttribute {
    pub(crate) request_type: RequestType,
    pub(crate) url: String,
}

impl Parse for RequestAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        
        let mut request_type = None;
        let mut url = None;
        
        loop {
            if input.is_empty() {
                break;
            }

            let key_span = input.span();
            let key: Ident = input.call(Ident::parse_any)?;
            let key = key.to_string();
            let _: Token![=] = input.parse()?;
            let value_span = input.span();
            let old = match key.as_str() {
                "type" => request_type.replace(input.parse::<LitStr>()?.value().parse().ok().ok_or_else(|| syn::Error::new(value_span, "Invalid request type"))?).map(|_| ()),
                "url" => url.replace(input.parse::<LitStr>()?.value()).map(|_| ()),
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
            request_type: request_type.ok_or(syn::Error::new(span, "No 'type' entry found"))?,
            url: url.ok_or(syn::Error::new(span, "No 'url' entry found"))?,
        })
    }
}

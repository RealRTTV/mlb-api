use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Fields, FieldsNamed, ItemStruct, Meta, Token};

pub(crate) struct FilterInput(pub(crate) ItemStruct);

fn is_list_keep_attr(attr: &Attribute) -> bool {
	if let Meta::List(list) = &attr.meta
		&& list.path.get_ident().is_some_and(|ident| ident.to_string() == "keep")
	{
		true
	} else {
		false
	}
}

fn is_path_keep_attr(attr: &Attribute) -> bool {
	if let Meta::Path(path) = &attr.meta
		&& path.get_ident().is_some_and(|ident| ident.to_string() == "keep")
	{
		true
	} else {
		false
	}
}

impl Parse for FilterInput {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let layout = input.parse::<ItemStruct>()?;
		let span = layout.span();

		let (keep_attr, attrs) = layout.attrs.into_iter().partition::<Vec<_>, _>(is_list_keep_attr);
		let Ok([Attribute { meta: Meta::List(keep_attr), .. }]) = TryInto::<[Attribute; 1]>::try_into(keep_attr) else {
			return Err(syn::Error::new(span, "Expected exactly one `keep` attribute (of type MetaList)"));
		};
		let keep_attr_list = Punctuated::<Ident, Token![,]>::parse_terminated.parse2(keep_attr.tokens)?.into_iter().collect::<Vec<Ident>>();

		let Fields::Named(FieldsNamed { brace_token, named: fields }) = layout.fields else {
			return Err(syn::Error::new(span, "Expected named fields"));
		};
		let (mut kept_fields, mut remaining_fields) = fields.into_iter().partition::<Vec<_>, _>(|field| field.attrs.iter().any(is_path_keep_attr));

		remaining_fields.retain_mut(|field| field.ident.as_ref().is_some_and(|ident| keep_attr_list.contains(ident)));
		for kept_field in &mut kept_fields {
			kept_field.attrs.retain(|attr| !is_path_keep_attr(attr))
		}
		kept_fields.append(&mut remaining_fields);

		Ok(Self(ItemStruct {
			attrs,
			vis: layout.vis,
			struct_token: layout.struct_token,
			ident: layout.ident,
			generics: layout.generics,
			fields: Fields::Named(FieldsNamed {
				brace_token,
				named: Punctuated::from_iter(kept_fields.into_iter()),
			}),
			semi_token: layout.semi_token,
		}))
	}
}

impl ToTokens for FilterInput {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens)
	}
}

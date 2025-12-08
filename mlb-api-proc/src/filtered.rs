use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Brace;
use syn::{Attribute, Field, Fields, FieldsNamed, Generics, ItemStruct, Meta, Token, Visibility};

pub(crate) struct FilterInput {
	other_attrs: Vec<Attribute>,
	struct_token: Token![struct],
	vis: Visibility,
	ident: Ident,
	generics: Generics,
	semi_token: Option<Token![;]>,

	fields_brace_token: Brace,
	fields: Vec<Field>,
}

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

		kept_fields.retain_mut(|field| if field.ident.as_ref().is_some_and(|ident| keep_attr_list.contains(ident)) { field.attrs.retain(|attr| !is_path_keep_attr(attr)); true } else { false });
		kept_fields.append(&mut remaining_fields);

		Ok(Self {
			other_attrs: attrs,
			struct_token: layout.struct_token,
			vis: layout.vis,
			ident: layout.ident,
			generics: layout.generics,
			semi_token: layout.semi_token,

			fields_brace_token: brace_token,
			fields: kept_fields,
		})
	}
}

impl ToTokens for FilterInput {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		ItemStruct {
			attrs: self.other_attrs.clone(),
			vis: self.vis.clone(),
			struct_token: self.struct_token.clone(),
			ident: self.ident.clone(),
			generics: self.generics.clone(),
			fields: Fields::Named(FieldsNamed {
				brace_token: self.fields_brace_token,
				named: Punctuated::from_iter(self.fields.clone().into_iter()),
			}),
			semi_token: self.semi_token.clone(),
		}
		.to_tokens(tokens)
	}
}

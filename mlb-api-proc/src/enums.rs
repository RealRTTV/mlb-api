use crate::pascal_or_camel_case_to_snake_case;
use anyhow::{anyhow, bail, ensure, Context, Result};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{Data, DataEnum, DeriveInput, Expr, ExprLit, Fields, FieldsUnnamed, GenericParam, Lit, Variant};

fn generics(input: &DeriveInput) -> (TokenStream, TokenStream) {
	let generics_def = input.generics.params.iter().map(|param| match param {
		GenericParam::Lifetime(param) => param.to_token_stream(),
		GenericParam::Type(param) => {
			let mut param = param.clone();
			param.default = None;
			param.into_token_stream()
		}
		GenericParam::Const(param) => {
			let mut param = param.clone();
			param.default = None;
			param.into_token_stream()
		}
	});
	let generics_def = quote! { < #(#generics_def),*> };
	let generics_use = input.generics.params.iter().map(|param| match param {
		GenericParam::Lifetime(param) => param.lifetime.to_token_stream(),
		GenericParam::Type(param) => param.ident.to_token_stream(),
		GenericParam::Const(param) => param.ident.to_token_stream(),
	});
	let generics_use = quote! { < #(#generics_use),* > };

	(generics_def, generics_use)
}

pub(crate) fn try_as(input: DeriveInput) -> Result<TokenStream> {
	let (generics_def, generics_use) = generics(&input);

	let name = input.ident;
	let Data::Enum(DataEnum { variants, .. }) = input.data else { bail!("EnumTryAs only works for enums") };

	let mut functions = TokenStream::new();

	for (idx, variant) in variants.iter().enumerate() {
		let variant_name = &variant.ident;
		let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields else { bail!("Enum variants must be unnamed") };
		ensure!(unnamed.len() == 1, "Enum variants must have exactly one unnamed field");
		let variant_type = &unnamed.iter().next().expect("At least one field").ty;

		let function_name = format_ident!("try_as_{}", pascal_or_camel_case_to_snake_case(variant_name.to_string()));
		let cases = variants.iter().enumerate().map(|(jdx, variant)| {
			let arm_variant_name = &variant.ident;
			let expr = if jdx <= idx {
				quote! { ::core::option::Option::Some(_x) }
			} else {
				quote! { ::core::option::Option::None }
			};

			quote! {
				Self::#arm_variant_name(_x) => #expr,
			}
		}).collect::<TokenStream>();

		let function = quote! {
			#[must_use]
			pub fn #function_name(&self) -> ::core::option::Option<& #variant_type> {
				match self {
					#cases
				}
			}
		};

		functions.append_all(function);
	}

	Ok(quote! {
		impl #generics_def #name #generics_use {
			#functions
		}
	})
}

pub(crate) fn try_as_mut(input: DeriveInput) -> Result<TokenStream> {
	let (generics_def, generics_use) = generics(&input);

	let name = input.ident;
	let Data::Enum(DataEnum { variants, .. }) = input.data else { bail!("EnumTryAs only works for enums") };

	let mut functions = TokenStream::new();

	for (idx, variant) in variants.iter().enumerate() {
		let variant_name = &variant.ident;
		let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields else { bail!("Enum variants must be unnamed") };
		ensure!(unnamed.len() == 1, "Enum variants must have exactly one unnamed field");
		let variant_type = &unnamed.iter().next().expect("At least one field").ty;

		let function_name = format_ident!("try_as_mut_{}", pascal_or_camel_case_to_snake_case(variant_name.to_string()));
		let cases = variants.iter().enumerate().map(|(jdx, variant)| {
			let arm_variant_name = &variant.ident;
			let expr = if jdx <= idx {
				quote! { ::core::option::Option::Some(_x) }
			} else {
				quote! { ::core::option::Option::None }
			};

			quote! {
				Self::#arm_variant_name(_x) => #expr,
			}
		}).collect::<TokenStream>();

		let function = quote! {
			#[must_use]
			pub fn #function_name(&mut self) -> ::core::option::Option<&mut #variant_type> {
				match self {
					#cases
				}
			}
		};

		functions.append_all(function);
	}

	Ok(quote! {
		impl #generics_def #name #generics_use {
			#functions
		}
	})
}

pub(crate) fn try_into(input: DeriveInput) -> Result<TokenStream> {
	let (generics_def, generics_use) = generics(&input);

	let inner_field_name = input
		.attrs
		.iter()
		.filter_map(|attr| attr.meta.require_name_value().ok())
		.find(|x| x.path.is_ident("try_into_field_name"))
		.and_then(|x| { if let Expr::Lit(ExprLit { lit: Lit::Str(str), .. }) = &x.value { Some(str.value()) } else { None } })
		.unwrap_or_else(|| "inner".to_owned()).parse::<TokenStream>().map_err(|e| anyhow!("{e}"))?;

	let name = input.ident;
	let Data::Enum(DataEnum { variants, .. }) = input.data else { bail!("EnumTryAs only works for enums") };

	let mut functions = TokenStream::new();

	for (idx, variant) in variants.iter().enumerate() {
		let variant_name = &variant.ident;
		let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields else { bail!("Enum variants must be unnamed") };
		ensure!(unnamed.len() == 1, "Enum variants must have exactly one unnamed field");
		let variant_type = &unnamed.iter().next().expect("At least one field").ty;

		let function_name = format_ident!("try_into_{}", pascal_or_camel_case_to_snake_case(variant_name.to_string()));
		let cases = variants.iter().enumerate().map(|(jdx, variant)| {
			let arm_variant_name = &variant.ident;
			let expr = if jdx <= idx {
				let inners = std::iter::repeat(quote! { . #inner_field_name }).take(idx - jdx).collect::<TokenStream>();
				quote! { ::core::option::Option::Some(_x #inners) }
			} else {
				quote! { ::core::option::Option::None }
			};

			quote! {
				Self::#arm_variant_name(_x) => #expr,
			}
		}).collect::<TokenStream>();

		let function = quote! {
			#[must_use]
			pub fn #function_name(self) -> ::core::option::Option<#variant_type> {
				match self {
					#cases
				}
			}
		};

		functions.append_all(function);
	}

	Ok(quote! {
		impl #generics_def #name #generics_use {
			#functions
		}
	})
}

pub(crate) fn deref(input: DeriveInput) -> Result<TokenStream> {
	let (generics_def, generics_use) = generics(&input);

	let name = input.ident;

	let Data::Enum(DataEnum { variants, .. }) = input.data else { bail!("Expected an enum as input") };
	let Variant { fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }), .. } = variants.last().cloned().context("Must have at least one variant for deref")? else { bail!("Expected unnamed fields") };
	let [minimum_variant_type] = unnamed.into_iter().collect_array::<1>().context("Expected exactly one field")?;

	let arms = variants.into_iter().map(|var| {
		let arm_variant_name = &var.ident;

		quote! {
			Self::#arm_variant_name(inner) => inner,
		}
	}).collect::<TokenStream>();

	Ok(quote! {
		impl #generics_def ::core::ops::Deref for #name #generics_use {
			type Target = #minimum_variant_type;

			fn deref(&self) -> &Self::Target {
				match self {
					#arms
				}
			}
		}
	})
}

pub(crate) fn deref_mut(input: DeriveInput) -> Result<TokenStream> {
	let (generics_def, generics_use) = generics(&input);

	let name = input.ident;

	let Data::Enum(DataEnum { variants, .. }) = input.data else { bail!("Expected an enum as input") };

	let arms = variants.into_iter().map(|var| {
		let arm_variant_name = &var.ident;

		quote! {
			Self::#arm_variant_name(inner) => inner,
		}
	}).collect::<TokenStream>();

	Ok(quote! {
		impl #generics_def ::core::ops::DerefMut for #name #generics_use {
			fn deref_mut(&mut self) -> &mut Self::Target {
				match self {
					#arms
				}
			}
		}
	})
}

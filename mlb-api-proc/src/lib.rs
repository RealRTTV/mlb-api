mod enums;
mod filtered;
mod stats;

use itertools::{Itertools, Position};
use proc_macro2::{Ident, Span};
use quote::{ToTokens, format_ident, quote};
use syn::{DeriveInput, LitStr};

#[proc_macro]
pub fn stats(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as stats::StatsInput);
	input.into_token_stream().into()
}

#[proc_macro]
pub fn concat_camel_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let literal = LitStr::new(&input.into_iter().map(|tt| snake_case_to_camel_case0(tt.to_string())).join(","), Span::call_site());
	quote! {
		#literal
	}
	.into()
}

#[proc_macro_attribute]
pub fn filter_fields(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(item as filtered::FilterInput);
	input.into_token_stream().into()
}

#[proc_macro]
pub fn pascal_case_to_camel_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let ident = syn::parse_macro_input!(input as Ident);
	let camel_case = pascal_case_to_camel_case0(ident.to_string());
	format_ident!("{camel_case}").into_token_stream().into()
}

#[proc_macro_derive(EnumTryAs)]
pub fn enum_try_as(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	enums::try_as(input).unwrap().into()
}

#[proc_macro_derive(EnumTryAsMut)]
pub fn enum_try_as_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	enums::try_as_mut(input).unwrap().into()
}

#[proc_macro_derive(EnumTryInto, attributes(try_into_field_name))]
pub fn enum_try_into(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	enums::try_into(input).unwrap().into()
}

#[proc_macro_derive(EnumDeref)]
pub fn enum_deref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	enums::deref(input).unwrap().into()
}

#[proc_macro_derive(EnumDerefMut)]
pub fn enum_deref_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	enums::deref_mut(input).unwrap().into()
}

fn pascal_case_to_camel_case0(pascal_case: String) -> String {
	pascal_case
		.chars()
		.with_position()
		.map(|(pos, char)| match pos {
			Position::First | Position::Only => char.to_ascii_lowercase(),
			Position::Middle | Position::Last => char,
		})
		.collect::<String>()
}

#[proc_macro]
pub fn snake_case_to_camel_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let ident = syn::parse_macro_input!(input as Ident);
	let camel_case = snake_case_to_camel_case0(ident.to_string());
	format_ident!("{camel_case}").into_token_stream().into()
}

#[rustfmt::skip]
fn pascal_or_camel_case_to_snake_case(pascal_case: String) -> String {
	fn is_breakpoint(c: char) -> bool {
		!c.is_ascii_lowercase()
	}

	let mut char_iter = pascal_case.chars().peekable();
	let mut prev_char: Option<char> = None;

	let mut builder = String::with_capacity(pascal_case.len());

	while let Some(current_char) = char_iter.next() {
		let next_char = char_iter.peek().copied();
		let is_prev_char_word_breakpoint = prev_char.map(is_breakpoint);
		let is_current_char_word_breakpoint = is_breakpoint(current_char);
		let is_next_char_word_breakpoint = next_char.map(is_breakpoint);

		let char = current_char.to_ascii_lowercase();

		match (is_prev_char_word_breakpoint, is_current_char_word_breakpoint, is_next_char_word_breakpoint) {
			//          v - previous
			//           v - current
			//            v - next
			/* case of:  a  */ (None, false, None) => builder.push(char),
			/* case of:  aa */ (None, false, Some(false)) => builder.push(char),
			/* case of:  aA */ (None, false, Some(true)) => { builder.push(char); builder.push('_') },
			/* case of:  A  */ (None, true, None) => builder.push(char),
			/* case of:  Aa */ (None, true, Some(false)) => builder.push(char),
			/* case of:  AA */ (None, true, Some(true)) => builder.push(char),
			/* case of: aa  */ (Some(false), false, None) => builder.push(char),
			/* case of: aaa */ (Some(false), false, Some(false)) => builder.push(char),
			/* case of: aaA */ (Some(false), false, Some(true)) => { builder.push(char); builder.push('_') },
			/* case of: aA  */ (Some(false), true, None) => builder.push(char),
			/* case of: aAa */ (Some(false), true, Some(false)) => builder.push(char),
			/* case of: aAA */ (Some(false), true, Some(true)) => builder.push(char),
			/* case of: Aa  */ (Some(true), false, None) => builder.push(char),
			/* case of: Aaa */ (Some(true), false, Some(false)) => builder.push(char),
			/* case of: AaA */ (Some(true), false, Some(true)) => { builder.push(char); builder .push('_') },
			/* case of: AA  */ (Some(true), true, None) => builder.push(char),
			/* case of: AAa */ (Some(true), true, Some(false)) => { builder.push('_'); builder.push(char) },
			/* case of: AAA */ (Some(true), true, Some(true)) => builder.push(char),
		}

		prev_char = Some(current_char);
	}

	builder
}

fn snake_case_to_camel_case0(snake_case: String) -> String {
	let mut char_iter = snake_case.chars();

	let mut builder = String::new();

	let mut capitalize = false;

	while let Some(char) = char_iter.next() {
		if char == '_' {
			capitalize = true;
		} else {
			let char = if capitalize { char.to_ascii_uppercase() } else { char };
			capitalize = false;
			builder.push(char);
		}
	}

	builder
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pascal_case_to_snake_case0() {
		assert_eq!(pascal_or_camel_case_to_snake_case("".to_owned()), "".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("HelloWorld".to_owned()), "hello_world".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("AaaaaAaaaa".to_owned()), "aaaaa_aaaaa".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("One".to_owned()), "one".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("Aaa".to_owned()), "aaa".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("OneTwoThree".to_owned()), "one_two_three".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("AaaAaaAaaaa".to_owned()), "aaa_aaa_aaaaa".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("VsPlayer5Y".to_owned()), "vs_player_5y".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("AaAaaaaaAA".to_owned()), "aa_aaaaaa_aa".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("TBDVariableThing".to_owned()), "tbd_variable_thing".to_owned());
		assert_eq!(pascal_or_camel_case_to_snake_case("AAAAaaaaaaaAaaaa".to_owned()), "aaa_aaaaaaaa_aaaaa".to_owned());
	}
}

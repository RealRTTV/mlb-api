use std::borrow::Cow;
use std::collections::HashSet;
use derive_syn_parse::Parse;
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{token, LitStr, Path, Token, Visibility};
use syn::punctuated::Punctuated;
use crate::{pascal_case_to_camel_case0, pascal_or_camel_case_to_snake_case};

fn to_mlb_api_stat_type(stat_type: Cow<'_, str>) -> String {
	match stat_type.as_ref() {
		"SimplifiedGameLog" => "GameLog",
		"SimplifiedPlayLog" => "PlayLog",
		x => x,
	}.to_owned()
}

#[rustfmt::skip]
#[derive(Parse)]
pub(crate)struct StatsInput {
	crate_name: Path,
	vis: Visibility,
	#[prefix(Token![struct])]
	name: Ident,
	#[brace]
	_brace_token: token::Brace,
		#[call(Punctuated::parse_terminated)]
		#[inside(_brace_token)]
		stat_types: Punctuated<StatTypeStatGroupList, Token![,]>,
}

#[rustfmt::skip]
#[derive(Parse)]
struct StatTypeStatGroupList {
	#[bracket]
	_bracket_token1: token::Bracket,
		#[inside(_bracket_token1)]
		#[call(Punctuated::parse_terminated)]
		stat_types: Punctuated<Ident, Token![,]>,

	_eq_token: Token![=],

	#[bracket]
	_bracket_token2: token::Bracket,
		#[inside(_bracket_token2)]
		#[call(Punctuated::parse_terminated)]
		stat_groups: Punctuated<Ident, Token![,]>,
}

impl ToTokens for StatsInput {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self {
			crate_name,
			vis,
			name,
			_brace_token: _,
			stat_types,
		} = self;

		let split_tokens: TokenStream = stat_types.iter().map(|entry| entry.stat_types.iter().map(|stat_type| {
			let stat_type_variant_name = format_ident!("{name}{}Split", stat_type);
			let stat_groups: TokenStream = entry.stat_groups.iter().map(|stat_group| {
				let snake_case_name = format_ident!("{}", pascal_or_camel_case_to_snake_case(stat_group.to_string()));
				let stat_type_stats = format_ident!("{}Stats", stat_type);
				quote! {
					#snake_case_name: Box<<#crate_name::endpoints::stats::#stat_type_stats as #crate_name::endpoints::stats::StatTypeStats>::#stat_group>,
				}
			}).collect::<TokenStream>();
			quote! {
				#[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
				#vis struct #stat_type_variant_name {
					#stat_groups
				}
			}
		}).collect::<TokenStream>()).collect::<TokenStream>();

		let stat_type_fields: TokenStream = stat_types.iter().map(|entry| entry.stat_types.iter().map(|stat_type| {
			let stat_type_variant_name = format_ident!("{name}{}Split", stat_type);
			let stat_type_field_name = format_ident!("{}", pascal_or_camel_case_to_snake_case(stat_type.to_string()));
			quote! {
				#stat_type_field_name: #stat_type_variant_name,
			}
		}).collect::<TokenStream>()).collect::<TokenStream>();

		let stat_type_fields_try_from: TokenStream = stat_types.iter().map(|entry| entry.stat_types.iter().map(|stat_type| {
			let stat_type_variant_name = format_ident!("{name}{}Split", stat_type);
			let stat_type_field_name = format_ident!("{}", pascal_or_camel_case_to_snake_case(stat_type.to_string()));
			let stat_group_fields_try_from: TokenStream = entry.stat_groups.iter().map(|stat_group| {
				let snake_case_name = format_ident!("{}", pascal_or_camel_case_to_snake_case(stat_group.to_string()));
				let stat_type_stats = format_ident!("{}Stats", stat_type);
				let stat_type_str_lit = LitStr::new(&to_mlb_api_stat_type(stat_type.to_string().into()), stat_type.span());
				quote! {
					#snake_case_name: Box::new(#crate_name::endpoints::stats::make_stat_split::<<#crate_name::endpoints::stats::#stat_type_stats as #crate_name::endpoints::stats::StatTypeStats>::#stat_group>(&mut value, #stat_type_str_lit, #crate_name::endpoints::StatGroup::#stat_group).map_err(|e| e.to_string())?),
				}
			}).collect::<TokenStream>();
			quote! {
				#stat_type_field_name: #stat_type_variant_name {
					#stat_group_fields_try_from
				},
			}
		}).collect::<TokenStream>()).collect::<TokenStream>();

		let request_text: String = {
			let all_stat_groups = stat_types.iter().flat_map(|stat_type| &stat_type.stat_groups).map(|stat_group| stat_group.to_string()).map(pascal_case_to_camel_case0).collect::<HashSet<_>>();
			let all_stat_types = stat_types.iter().flat_map(|stat_type| &stat_type.stat_types).map(|stat_type| stat_type.to_string()).map(|s| to_mlb_api_stat_type(pascal_case_to_camel_case0(s).into())).collect::<HashSet<_>>();
			format!("group=[{}],type=[{}]", all_stat_groups.into_iter().join(","), all_stat_types.into_iter().join(","))
		};
		let request_text_lit: LitStr = LitStr::new(&request_text, Span::call_site());

		tokens.append_all(quote! {
			#[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
			#vis struct #name {
				#stat_type_fields
			}

			impl<'de> ::serde::Deserialize<'de> for #name {
				fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
				where
					D: ::serde::Deserializer<'de>
				{
					::core::result::Result::and_then(<#crate_name::endpoints::stats::__ParsedStats as ::serde::Deserialize>::deserialize(deserializer), |v| ::core::convert::TryFrom::try_from(v).map_err(::serde::de::Error::custom))
				}
			}

			impl #crate_name::endpoints::stats::Stats for #name {
				fn request_text() -> &'static str {
					#request_text_lit
				}
			}

			impl ::core::convert::TryFrom<#crate_name::endpoints::stats::__ParsedStats> for #name {
				type Error = ::std::string::String;

				fn try_from(mut value: #crate_name::endpoints::stats::__ParsedStats) -> ::core::result::Result<Self, Self::Error> {
					::core::result::Result::Ok(Self {
						#stat_type_fields_try_from
					})
				}
			}

			#split_tokens
		})
	}
}

use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use derive_more::Display;
use serde::Deserialize;

/// Different types of standings
#[derive(Deserialize, Default,PartialEq, Eq, Copy, Clone, Display, Hash)]
#[serde(try_from = "__StandingsTypeStruct")]
pub enum StandingsType {
	///	Regular Season Standings
	#[default]
	#[display("Regular Season")]
	RegularSeason,

	///	Wild card standings
	#[display("Wild Card")]
	WildCard,

	///	Division Leader standings
	#[display("Division Leaders")]
	DivisionLeaders,

	///	Wild card standings with Division Leaders
	#[display("Wild Card With Leaders")]
	WildCardWithLeaders,

	///	First half standings.  Only valid for leagues with a split season.
	#[display("First Half")]
	FirstHalf,

	///	Second half standings. Only valid for leagues with a split season.
	#[display("Second Half")]
	SecondHalf,

	///	Spring Training Standings
	#[display("Spring Training")]
	SpringTraining,

	///	Postseason Standings
	#[display("Postseason")]
	Postseason,

	///	Standings by Division
	#[display("By Division")]
	ByDivision,

	///	Standings by Conference
	#[display("By Conference")]
	ByConference,

	///	Standings by League
	#[display("By League")]
	ByLeague,

	///	Standing by Organization
	#[display("By Organization")]
	ByOrganization,

	///	Current Half Standings. Returns standings in the current half for split season leagues and overall standings for full season leagues.
	#[display("Current Half")]
	CurrentHalf,
}

impl Debug for StandingsType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::RegularSeason => "regularSeason",
				Self::WildCard => "wildCard",
				Self::DivisionLeaders => "divisionLeaders",
				Self::WildCardWithLeaders => "wildCardWithLeaders",
				Self::FirstHalf => "firstHalf",
				Self::SecondHalf => "secondHalf",
				Self::SpringTraining => "springTraining",
				Self::Postseason => "postseason",
				Self::ByDivision => "byDivision",
				Self::ByConference => "byConference",
				Self::ByLeague => "byLeague",
				Self::ByOrganization => "byOrganization",
				Self::CurrentHalf => "currentHalf",
			}
		)
	}
}

#[derive(Deserialize)]
#[doc(hidden)]
#[serde(untagged)]
enum __StandingsTypeStruct {
	Wrapped {
		name: String,
	},
	Inline(String),
}

impl Deref for __StandingsTypeStruct {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		let (Self::Wrapped { name, .. } | Self::Inline(name)) = self;
		name
	}
}

impl TryFrom<__StandingsTypeStruct> for StandingsType {
	type Error = &'static str;

	fn try_from(value: __StandingsTypeStruct) -> Result<Self, Self::Error> {
		Ok(match &**value {
			"regularSeason" => Self::RegularSeason,
			"wildCard" => Self::WildCard,
			"divisionLeaders" => Self::DivisionLeaders,
			"wildCardWithLeaders" => Self::WildCardWithLeaders,
			"firstHalf" => Self::FirstHalf,
			"secondHalf" => Self::SecondHalf,
			"springTraining" => Self::SpringTraining,
			"postseason" => Self::Postseason,
			"byDivision" => Self::ByDivision,
			"byConference" => Self::ByConference,
			"byLeague" => Self::ByLeague,
			"byOrganization" => Self::ByOrganization,
			"currentHalf" => Self::CurrentHalf,
			_ => return Err("unknown standings type"),
		})
	}
}

meta_kind_impl!("standingsTypes" => StandingsType);
static_request_entry_cache_impl!(StandingsType);
test_impl!(StandingsType);

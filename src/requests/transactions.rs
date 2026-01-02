use crate::person::{NamedPerson, PersonId};
use crate::team::{TeamId};
use crate::types::{Copyright, NaiveDateRange, MLB_API_DATE_FORMAT};
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use chrono::NaiveDate;
use itertools::Itertools;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use crate::sport::SportId;
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsResponse {
	pub copyright: Copyright,
	pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCommon {
	pub id: TransactionId,
	#[serde(default)]
	pub description: String,
	#[serde(flatten)]
	pub dates: TransactionDates,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDates {
	pub date: NaiveDate,
	pub effective_date: Option<NaiveDate>,
	pub resolution_date: Option<NaiveDate>,
}

//noinspection DuplicatedCode
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "typeCode")]
pub enum Transaction {
	#[serde(rename = "ASG", rename_all = "camelCase")]
	Assigned {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team_opt")]
		#[serde(default)]
		source_team: Option<NamedTeam>,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "SC", rename_all = "camelCase")]
	StatusChange {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "SFA", rename_all = "camelCase")]
	SignedAsFreeAgent {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "DES", rename_all = "camelCase")]
	DesignatedForAssignment {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "TR", rename_all = "camelCase")]
	Trade {
		#[serde(flatten)]
		common: TransactionCommon,
		/// No person here indicates a trade occurred that gave the team cash.
		#[serde(deserialize_with = "deserialize_named_person_opt")]
		#[serde(default)]
		person: Option<NamedPerson>,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "NUM", rename_all = "camelCase")]
	NumberChange {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "OUT", rename_all = "camelCase")]
	Outrighted {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "CLW", rename_all = "camelCase")]
	ClaimedOffWaivers {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "SGN", rename_all = "camelCase")]
	Signed {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "REL", rename_all = "camelCase")]
	Released {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "DFA", rename_all = "camelCase")]
	DeclaredFreeAgency {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
	},
	#[serde(rename = "OPT", rename_all = "camelCase")]
	Optioned {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "RTN", rename_all = "camelCase")]
	Returned {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "SE", rename_all = "camelCase")]
	Selected {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team_opt")]
		#[serde(default)]
		source_team: Option<NamedTeam>,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "CU", rename_all = "camelCase")]
	Recalled {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team_opt")]
		#[serde(default)]
		source_team: Option<NamedTeam>,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "SU", rename_all = "camelCase")]
	Suspension {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "RET", rename_all = "camelCase")]
	Retired {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "PUR", rename_all = "camelCase")]
	Purchase {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team_opt")]
		#[serde(default)]
		source_team: Option<NamedTeam>,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "R5", rename_all = "camelCase")]
	RuleFiveDraft {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "RE", rename_all = "camelCase")]
	Reinstated {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "LON", rename_all = "camelCase")]
	Loan {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "CP", rename_all = "camelCase")]
	ContractPurchased {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "DR", rename_all = "camelCase")]
	Drafted {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "DEI", rename_all = "camelCase")]
	DeclaredIneligible {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
	#[serde(rename = "R5M", rename_all = "camelCase")]
	RuleFiveDraftMinors {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "fromTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		source_team: NamedTeam,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		destination_team: NamedTeam,
	},
	#[serde(rename = "RES", rename_all = "camelCase")]
	Reserved {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(deserialize_with = "deserialize_named_person")]
		#[serde(default = "NamedPerson::unknown_person")]
		person: NamedPerson,
		#[serde(default = "NamedTeam::unknown_team")]
		#[serde(rename = "toTeam")]
		#[serde(deserialize_with = "deserialize_named_team")]
		team: NamedTeam,
	},
}

impl Deref for Transaction {
	type Target = TransactionCommon;

	#[rustfmt::skip]
	fn deref(&self) -> &Self::Target {
		match self {
			Self::Assigned { common, .. }
			| Self::StatusChange { common, .. }
			| Self::SignedAsFreeAgent { common, .. }
			| Self::DesignatedForAssignment { common, .. }
			| Self::Trade { common, .. }
			| Self::NumberChange { common, .. }
			| Self::Outrighted { common, .. }
			| Self::ClaimedOffWaivers { common, .. }
			| Self::Signed { common, .. }
			| Self::Released { common, .. }
			| Self::DeclaredFreeAgency { common, .. }
			| Self::Optioned { common, .. }
			| Self::Returned { common, .. }
			| Self::Selected { common, .. }
			| Self::Recalled { common, .. }
			| Self::Suspension { common, .. }
			| Self::Retired { common, .. }
			| Self::Purchase { common, .. }
			| Self::RuleFiveDraft { common, .. }
			| Self::Reinstated { common, .. }
			| Self::Loan { common, .. }
			| Self::ContractPurchased { common, .. }
			| Self::Drafted { common, .. }
			| Self::DeclaredIneligible { common, .. }
			| Self::RuleFiveDraftMinors { common, .. }
			| Self::Reserved { common, .. } => common,
		}
	}
}

impl DerefMut for Transaction {
	#[rustfmt::skip]
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Assigned { common, .. }
			| Self::StatusChange { common, .. }
			| Self::SignedAsFreeAgent { common, .. }
			| Self::DesignatedForAssignment { common, .. }
			| Self::Trade { common, .. }
			| Self::NumberChange { common, .. }
			| Self::Outrighted { common, .. }
			| Self::ClaimedOffWaivers { common, .. }
			| Self::Signed { common, .. }
			| Self::Released { common, .. }
			| Self::DeclaredFreeAgency { common, .. }
			| Self::Optioned { common, .. }
			| Self::Returned { common, .. }
			| Self::Selected { common, .. }
			| Self::Recalled { common, .. }
			| Self::Suspension { common, .. }
			| Self::Retired { common, .. }
			| Self::Purchase { common, .. }
			| Self::RuleFiveDraft { common, .. }
			| Self::Reinstated { common, .. }
			| Self::Loan { common, .. }
			| Self::ContractPurchased { common, .. }
			| Self::Drafted { common, .. }
			| Self::DeclaredIneligible { common, .. }
			| Self::RuleFiveDraftMinors { common, .. }
			| Self::Reserved { common, .. } => common,
		}
	}
}

impl Display for Transaction {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

id!(TransactionId { id: u32 });

fn deserialize_named_person<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NamedPerson, D::Error> {
	#[derive(Deserialize)]
	#[serde(rename = "camelCase")]
	struct NamedPersonWrapper {
		#[serde(alias = "name")]
		full_name: Option<String>,
		#[serde(flatten)]
		id: Option<PersonId>,
	}

	let NamedPersonWrapper { full_name, id } = NamedPersonWrapper::deserialize(deserializer)?;
	Ok(NamedPerson {
		full_name: full_name.unwrap_or(String::new()),
		id: id.unwrap_or(PersonId::new(0))
	})
}

fn deserialize_named_person_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<NamedPerson>, D::Error> {
	#[derive(Deserialize)]
	#[serde(rename = "camelCase")]
	struct NamedPersonWrapper {
		#[serde(alias = "name")]
		full_name: Option<String>,
		#[serde(flatten)]
		id: Option<PersonId>,
	}

	let wrapped = NamedPersonWrapper::deserialize(deserializer)?;
	match wrapped {
		NamedPersonWrapper { full_name, id: Some(id) } => Ok(Some(NamedPerson { full_name: full_name.unwrap_or(String::new()), id })),
		_ => Ok(None)
	}
}

fn deserialize_named_team<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NamedTeam, D::Error> {
	#[derive(Deserialize)]
	#[serde(rename = "camelCase")]
	struct NamedTeamWrapper {
		#[serde(alias = "name")]
		full_name: Option<String>,
		#[serde(flatten)]
		id: Option<TeamId>,
	}

	let NamedTeamWrapper { full_name, id } = NamedTeamWrapper::deserialize(deserializer)?;
	Ok(NamedTeam {
		full_name: full_name.unwrap_or(String::new()),
		id: id.unwrap_or(TeamId::new(0))
	})
}

fn deserialize_named_team_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<NamedTeam>, D::Error> {
	#[derive(Deserialize)]
	#[serde(rename = "camelCase")]
	struct NamedTeamWrapper {
		#[serde(alias = "name")]
		full_name: Option<String>,
		#[serde(flatten)]
		id: Option<TeamId>,
	}

	let wrapped = NamedTeamWrapper::deserialize(deserializer)?;
	match wrapped {
		NamedTeamWrapper { full_name, id: Some(id) } => Ok(Some(NamedTeam { full_name: full_name.unwrap_or(String::new()), id })),
		_ => Ok(None),
	}
}

pub enum TransactionsRequestKind {
	Team(TeamId),
	Player(PersonId),
	Transactions(Vec<TransactionId>),
	DateRange(NaiveDateRange),
}

impl Display for TransactionsRequestKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Team(team_id) => write!(f, "teamId={team_id}"),
			Self::Player(person_id) => write!(f, "playerId={person_id}"),
			Self::Transactions(transactions) => write!(f, "transactionIds={}", transactions.iter().join(",")),
			Self::DateRange(range) => write!(f, "startDate={}&endDate={}", range.start().format(MLB_API_DATE_FORMAT), range.end().format(MLB_API_DATE_FORMAT)),
		}
	}
}

/// Sends a request to get the relevant transactions for a player.
///
/// This API request is somewhat unreliable. For an example of what I mean: <http://statsapi.mlb.com/api/v1/transactions?transactionIds=477955>. Vladimir Guerrero Jr.'s `.` in his name causes the API to be super confused and generate 5 players, four of which don't exist.
///
/// Of course putting [`Option<Person>`] for the `person` field is needlessly overkill since mostly all situations will not cause this, but the transactions shouldn't be discarded.
///
/// Instead, these values (no team, no date, no player) are given default values such that they are valid, but any further API requests with them return an error, such as a person with ID 0.
#[derive(Builder)]
#[builder(derive(Into))]
#[builder(start_fn(vis = "", name = "__builder_internal"))]
pub struct TransactionsRequest {
	#[builder(setters(vis = "", name = __kind_internal))]
	kind: TransactionsRequestKind,
	#[builder(into)]
	sport_id: Option<SportId>,
}

impl TransactionsRequest {
	pub fn for_team(team_id: impl Into<TeamId>) -> TransactionsRequestBuilder<transactions_request_builder::SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::Team(team_id.into()))
	}

	pub fn for_player(person_id: impl Into<PersonId>) -> TransactionsRequestBuilder<transactions_request_builder::SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::Player(person_id.into()))
	}

	pub fn for_ids(transactions: Vec<TransactionId>) -> TransactionsRequestBuilder<transactions_request_builder::SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::Transactions(transactions))
	}

	pub fn for_date_range(range: NaiveDateRange) -> TransactionsRequestBuilder<transactions_request_builder::SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::DateRange(range))
	}
}

impl<S: transactions_request_builder::State + transactions_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for TransactionsRequestBuilder<S> {
	type Built = TransactionsRequest;
}

impl Display for TransactionsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/transactions{}",
			gen_params! {
				self.kind,
				"sportId"?: self.sport_id,
			}
		)
	}
}

impl StatsAPIRequestUrl for TransactionsRequest {
	type Response = TransactionsResponse;
}

#[cfg(test)]
mod tests {
	use crate::person::players::PlayersRequest;
	use crate::team::teams::TeamsRequest;
	use crate::transactions::TransactionsRequest;
	use crate::TEST_YEAR;
	use chrono::NaiveDate;
	use crate::request::StatsAPIRequestUrlBuilderExt;
	use crate::sport::SportId;

	#[tokio::test]
	async fn parse_current_year() {
		let _ = TransactionsRequest::for_date_range(NaiveDate::from_ymd_opt(TEST_YEAR.try_into().unwrap(), 1, 1).unwrap()..=NaiveDate::from_ymd_opt(TEST_YEAR.try_into().unwrap(), 12, 31).unwrap()).sport_id(SportId::MLB).build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn test_single_transaction() {
		let _ = TransactionsRequest::for_ids(vec![809_972.into()]).build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn parse_sample_requests() {
		let blue_jays = TeamsRequest::mlb_teams()
			.season(TEST_YEAR)
			.build_and_get()
			.await
			.unwrap()
			.teams
			.into_iter()
			.find(|team| team.name.as_str() == "Toronto Blue Jays")
			.unwrap();
		let bo_bichette = PlayersRequest::builder()
			.season(2024)
			.build_and_get()
			.await
			.unwrap()
			.people
			.into_iter()
			.find(|person| person.full_name == "Bo Bichette")
			.unwrap();

		let response = TransactionsRequest::for_date_range(NaiveDate::from_ymd_opt(TEST_YEAR.try_into().unwrap(), 1, 1).unwrap()..=NaiveDate::from_ymd_opt(TEST_YEAR.try_into().unwrap(), 12, 31).unwrap()).build_and_get().await.unwrap();
		let transaction_ids = response.transactions.into_iter().take(20).map(|transaction| transaction.id).collect::<Vec<_>>();
		let _response = TransactionsRequest::for_team(blue_jays.id).build_and_get().await.unwrap();
		let _response = TransactionsRequest::for_player(bo_bichette.id).build_and_get().await.unwrap();
		let _response = TransactionsRequest::for_ids(transaction_ids).build_and_get().await.unwrap();
	}
}

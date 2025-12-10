use crate::gen_params;
use crate::person::{Person, PersonId};
use crate::sports::SportId;
use crate::teams::team::{Team, TeamId};
use crate::types::{Copyright, NaiveDateRange, MLB_API_DATE_FORMAT};
use crate::StatsAPIRequestUrl;
use bon::Builder;
use chrono::NaiveDate;
use derive_more::{Deref, Display, From};
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "typeCode")]
pub enum Transaction {
	#[serde(rename = "ASG", rename_all = "camelCase")]
	Assigned {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Option<Team>,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "SC", rename_all = "camelCase")]
	StatusChange {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "SFA", rename_all = "camelCase")]
	SignedAsFreeAgent {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "DES", rename_all = "camelCase")]
	DesignatedForAssignment {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "TR", rename_all = "camelCase")]
	Trade {
		#[serde(flatten)]
		common: TransactionCommon,
		/// No person here indicates a trade occured that gave the team cash.
		person: Option<Person>,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "fromTeam")]
		source_team: Team,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "NUM", rename_all = "camelCase")]
	NumberChange {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "OUT", rename_all = "camelCase")]
	Outrighted {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Team,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "CLW", rename_all = "camelCase")]
	ClaimedOffWaivers {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "fromTeam")]
		source_team: Team,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "SGN", rename_all = "camelCase")]
	Signed {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "REL", rename_all = "camelCase")]
	Released {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "DFA", rename_all = "camelCase")]
	DeclaredFreeAgency {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		source_team: Team,
	},
	#[serde(rename = "OPT", rename_all = "camelCase")]
	Optioned {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Team,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "RTN", rename_all = "camelCase")]
	Returned {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "fromTeam")]
		source_team: Team,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "SE", rename_all = "camelCase")]
	Selected {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Option<Team>,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "CU", rename_all = "camelCase")]
	Recalled {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Option<Team>,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "SU", rename_all = "camelCase")]
	Suspension {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "RET", rename_all = "camelCase")]
	Retired {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "PUR", rename_all = "camelCase")]
	Purchase {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Option<Team>,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "R5", rename_all = "camelCase")]
	RuleFiveDraft {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Team,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "RE", rename_all = "camelCase")]
	Reinstated {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "LON", rename_all = "camelCase")]
	Loan {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(rename = "fromTeam")]
		source_team: Team,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		destination_team: Team,
	},
	#[serde(rename = "CP", rename_all = "camelCase")]
	ContractPurchased {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "DR", rename_all = "camelCase")]
	Drafted {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
	},
	#[serde(rename = "DEI", rename_all = "camelCase")]
	DeclaredIneligible {
		#[serde(flatten)]
		common: TransactionCommon,
		#[serde(default = "Person::unknown_person")]
		person: Person,
		#[serde(default = "Team::unknown_team")]
		#[serde(rename = "toTeam")]
		team: Team,
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
			| Self::DeclaredIneligible { common, .. } => common,
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
			| Self::DeclaredIneligible { common, .. } => common,
		}
	}
}

impl Display for Transaction {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.description)
	}
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct TransactionId(u32);

impl TransactionId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
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

use transactions_request_builder::SetKind;

impl TransactionsRequest {
	pub fn for_team(team_id: impl Into<TeamId>) -> TransactionsRequestBuilder<SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::Team(team_id.into()))
	}

	pub fn for_player(person_id: impl Into<PersonId>) -> TransactionsRequestBuilder<SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::Player(person_id.into()))
	}

	pub fn for_ids(transactions: Vec<TransactionId>) -> TransactionsRequestBuilder<SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::Transactions(transactions))
	}

	pub fn for_date_range(range: NaiveDateRange) -> TransactionsRequestBuilder<SetKind> {
		Self::__builder_internal().__kind_internal(TransactionsRequestKind::DateRange(range))
	}
}

impl<S: transactions_request_builder::State + transactions_request_builder::IsComplete> crate::requests::links::StatsAPIRequestUrlBuilderExt for TransactionsRequestBuilder<S> {
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
	use crate::person::Person;
	use crate::sports::players::SportsPlayersRequest;
	use crate::sports::SportId;
	use crate::teams::team::Team;
	use crate::teams::TeamsRequest;
	use crate::transactions::{TransactionsRequest, TransactionsRequestKind};
	use crate::StatsAPIRequestUrlBuilderExt;
	use chrono::NaiveDate;

	#[tokio::test]
	async fn parse_2025() {
		let _ = crate::serde_path_to_error_parse(TransactionsRequest {
			kind: TransactionsRequestKind::DateRange(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()..=NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
			sport_id: Some(SportId::MLB),
		})
		.await;
	}

	#[tokio::test]
	async fn parse_all_requests() {
		let blue_jays = TeamsRequest::builder()
			.season(2025)
			.build_and_get()
			.await
			.unwrap()
			.teams
			.into_iter()
			.filter_map(Team::try_into_named)
			.find(|team| team.name.as_str() == "Toronto Blue Jays")
			.unwrap();
		let bo_bichette = SportsPlayersRequest::builder()
			.season(2024)
			.build_and_get()
			.await
			.unwrap()
			.people
			.into_iter()
			.filter_map(Person::try_into_named)
			.find(|person| person.full_name == "Bo Bichette")
			.unwrap();

		let response = TransactionsRequest::for_date_range(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()..=NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
			.build_and_get()
			.await
			.unwrap();
		let transaction_ids = response.transactions.into_iter().take(1).map(|transaction| transaction.id).collect::<Vec<_>>();
		let _response = TransactionsRequest::for_team(blue_jays.id).build_and_get().await.unwrap();
		let _response = TransactionsRequest::for_player(bo_bichette.id).build_and_get().await.unwrap();
		let _response = TransactionsRequest::for_ids(transaction_ids).build_and_get().await.unwrap();
	}
}

use crate::endpoints::StatsAPIEndpointUrl;
use crate::endpoints::person::{Person, PersonId};
use crate::endpoints::sports::SportId;
use crate::endpoints::teams::team::{Team, TeamId};
use crate::gen_params;
use crate::types::{Copyright, MLB_API_DATE_FORMAT, NaiveDateRange};
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
	}
}

impl Deref for Transaction {
	type Target = TransactionCommon;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Assigned { common, .. } => common,
			Self::StatusChange { common, .. } => common,
			Self::SignedAsFreeAgent { common, .. } => common,
			Self::DesignatedForAssignment { common, .. } => common,
			Self::Trade { common, .. } => common,
			Self::NumberChange { common, .. } => common,
			Self::Outrighted { common, .. } => common,
			Self::ClaimedOffWaivers { common, .. } => common,
			Self::Signed { common, .. } => common,
			Self::Released { common, .. } => common,
			Self::DeclaredFreeAgency { common, .. } => common,
			Self::Optioned { common, .. } => common,
			Self::Returned { common, .. } => common,
			Self::Selected { common, .. } => common,
			Self::Recalled { common, .. } => common,
			Self::Suspension { common, .. } => common,
			Self::Retired { common, .. } => common,
			Self::Purchase { common, .. } => common,
			Self::RuleFiveDraft { common, .. } => common,
			Self::Reinstated { common, .. } => common,
			Self::Loan { common, .. } => common,
			Self::ContractPurchased { common, .. } => common,
			Self::Drafted { common, .. } => common,
			Self::DeclaredIneligible { common, .. } => common,
		}
	}
}

impl DerefMut for Transaction {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Assigned { common, .. } => common,
			Self::StatusChange { common, .. } => common,
			Self::SignedAsFreeAgent { common, .. } => common,
			Self::DesignatedForAssignment { common, .. } => common,
			Self::Trade { common, .. } => common,
			Self::NumberChange { common, .. } => common,
			Self::Outrighted { common, .. } => common,
			Self::ClaimedOffWaivers { common, .. } => common,
			Self::Signed { common, .. } => common,
			Self::Released { common, .. } => common,
			Self::DeclaredFreeAgency { common, .. } => common,
			Self::Optioned { common, .. } => common,
			Self::Returned { common, .. } => common,
			Self::Selected { common, .. } => common,
			Self::Recalled { common, .. } => common,
			Self::Suspension { common, .. } => common,
			Self::Retired { common, .. } => common,
			Self::Purchase { common, .. } => common,
			Self::RuleFiveDraft { common, .. } => common,
			Self::Reinstated { common, .. } => common,
			Self::Loan { common, .. } => common,
			Self::ContractPurchased { common, .. } => common,
			Self::Drafted { common, .. } => common,
			Self::DeclaredIneligible { common, .. } => common,
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

pub enum TransactionsEndpointKind {
	Team(TeamId),
	Player(PersonId),
	Transactions(Vec<TransactionId>),
	DateRange(NaiveDateRange),
}

impl Display for TransactionsEndpointKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Team(team_id) => write!(f, "teamId={team_id}"),
			Self::Player(person_id) => write!(f, "playerId={person_id}"),
			Self::Transactions(transactions) => write!(f, "transactionIds={}", transactions.iter().join(",")),
			Self::DateRange(range) => write!(f, "startDate={}&endDate={}", range.start().format(MLB_API_DATE_FORMAT), range.end().format(MLB_API_DATE_FORMAT)),
		}
	}
}

/// This API endpoint is rather unreliable. For an example of what I mean: http://statsapi.mlb.com/api/v1/transactions?transactionIds=477955 \
/// Vladimir Guerrero Jr.'s `.` in his name causes the API to be super confused and generate 5 players, four of which don't exist.\
/// Of course putting `[Option<Person>]` for the `person` field is needlessly overkill since mostly all situations will not cause this, but the transactions shouldn't be discarded.\
/// Instead, these values (no team, no date, no player) are given default values such that they are valid, but any further API requests with them return an error, such as a person with ID 0.
pub struct TransactionsEndpoint {
	pub kind: TransactionsEndpointKind,
	pub sport_id: Option<SportId>,
}

impl Display for TransactionsEndpoint {
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

impl StatsAPIEndpointUrl for TransactionsEndpoint {
	type Response = TransactionsResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIEndpointUrl;
	use crate::endpoints::sports::SportId;
	use crate::endpoints::sports::players::SportsPlayersEndpoint;
	use crate::endpoints::teams::TeamsEndpoint;
	use crate::endpoints::teams::team::Team;
	use crate::endpoints::transactions::{TransactionsEndpoint, TransactionsEndpointKind};
	use chrono::NaiveDate;
	use crate::endpoints::person::Person;

	#[tokio::test]
	async fn parse_2025() {
		let _ = crate::serde_path_to_error_parse(TransactionsEndpoint {
			kind: TransactionsEndpointKind::DateRange(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()..=NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
			sport_id: Some(SportId::MLB),
		}).await;
	}

	#[tokio::test]
	async fn parse_all_endpoints() {
		let blue_jays = TeamsEndpoint {
			sport_id: Some(SportId::MLB),
			season: Some(2025),
		}
		.get()
		.await
		.unwrap()
		.teams
		.into_iter()
		.filter_map(Team::try_as_named)
		.find(|team| team.name.as_str() == "Toronto Blue Jays")
		.unwrap();
		let bo_bichette = SportsPlayersEndpoint { id: SportId::MLB, season: Some(2025) }
			.get()
			.await
			.unwrap()
			.people
			.into_iter()
			.filter_map(Person::try_as_named)
			.find(|person| person.full_name == "Bo Bichette")
			.unwrap();

		let response = TransactionsEndpoint {
			kind: TransactionsEndpointKind::DateRange(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()..=NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
			sport_id: Some(SportId::MLB),
		}
		.get()
		.await
		.unwrap();
		let transaction_ids = response.transactions.into_iter().take(1).map(|transaction| transaction.id).collect::<Vec<_>>();
		let _response = TransactionsEndpoint {
			kind: TransactionsEndpointKind::Team(blue_jays.id),
			sport_id: Some(SportId::MLB),
		}
		.get()
		.await
		.unwrap();
		let _response = TransactionsEndpoint {
			kind: TransactionsEndpointKind::Player(bo_bichette.id),
			sport_id: Some(SportId::MLB),
		}
		.get()
		.await
		.unwrap();
		let _response = TransactionsEndpoint {
			kind: TransactionsEndpointKind::Transactions(transaction_ids),
			sport_id: Some(SportId::MLB),
		}
		.get()
		.await
		.unwrap();
	}
}

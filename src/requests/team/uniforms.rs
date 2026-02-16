use crate::cache::{Requestable, RequestableEntrypoint};
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::types::Copyright;
use crate::request::RequestURL;
use bon::Builder;
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[cfg(feature = "cache")]
use crate::{rwlock_const_new, RwLock, cache::CacheTable};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UniformsResponse {
    pub copyright: Copyright,
    #[serde(rename = "uniforms")] pub teams: Vec<TeamUniformAssets>,
}

id!(UniformAssetId { uniformAssetId: u32 });
id!(UniformAssetCategoryId { uniformAssetTypeId: u32 });

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamUniformAssets {
    pub team_id: TeamId,
    pub uniform_assets: Vec<UniformAsset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UniformAsset {
    #[serde(rename = "uniformAssetText")] pub name: String,
    #[serde(rename = "uniformAssetType")] pub category: UniformAssetCategory,
    #[serde(rename = "uniformAssetCode")] pub code: String,
    #[serde(flatten)]
    pub id: UniformAssetId,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UniformAssetCategory {
    #[serde(rename = "uniformAssetTypeText")] pub name: String,
    #[serde(rename = "uniformAssetTypeCode")] pub code: String,
    #[serde(rename = "uniformAssetTypeDesc")] pub description: String,
    #[serde(rename = "uniformAssetTypeId")] pub id: UniformAssetCategoryId,
}

id_only_eq_impl!(UniformAsset, id);
id_only_eq_impl!(UniformAssetCategory, id);

#[derive(Builder)]
#[builder(derive(Into))]
pub struct UniformsRequest {
    teams: Vec<TeamId>,
    #[builder(into)]
    season: Option<SeasonId>,
}

impl<S: uniforms_request_builder::State + uniforms_request_builder::IsComplete> crate::request::RequestURLBuilderExt for UniformsRequestBuilder<S> {
    type Built = UniformsRequest;
}

impl Display for UniformsRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/uniforms/team{}", gen_params! { "teamIds": self.teams.iter().copied().join(","), "season"?: self.season })
    }
}

impl RequestURL for UniformsRequest {
    type Response = UniformsResponse;
}

#[cfg(feature = "cache")]
static CACHE: RwLock<CacheTable<UniformAsset>> = rwlock_const_new(CacheTable::new());

impl Requestable for UniformAsset {
    type Identifier = String;
    type URL = UniformsRequest;

    fn id(&self) -> &Self::Identifier {
        &self.code
    }

    fn url_for_id(id: &Self::Identifier) -> Self::URL {
        UniformsRequest::builder()
            .teams(vec![TeamId::new(id.split_once('_').and_then(|(num, _)| num.parse().ok()).unwrap_or(0))])
            .build()
    }

    fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item=Self>
    where
        Self: Sized
    {
        response.teams.into_iter().flat_map(|team| team.uniform_assets)
    }

    #[cfg(feature = "cache")]
    fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
    where
        Self: Sized
    {
        &CACHE
    }
}

impl RequestableEntrypoint for UniformAsset {
    type Complete = Self;

    fn id(&self) -> &<<Self as RequestableEntrypoint>::Complete as Requestable>::Identifier {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use crate::team::uniforms::UniformsRequest;
    use crate::team::teams::TeamsRequest;
    use crate::request::RequestURLBuilderExt;

    #[tokio::test]
    async fn parse_all_mlb_teams_this_season() {
        let mlb_teams = TeamsRequest::mlb_teams().build_and_get().await.unwrap();
        let team_ids = mlb_teams.teams.into_iter().map(|team| team.id).collect::<Vec<_>>();
        let _ = UniformsRequest::builder().teams(team_ids).build_and_get().await.unwrap();
    }
}

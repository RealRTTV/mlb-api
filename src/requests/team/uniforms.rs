use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::season::SeasonId;
use crate::requests::team::TeamId;
use crate::types::Copyright;
use crate::{rwlock_const_new, RwLock};
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use derive_more::{Deref, DerefMut, From};
use itertools::Itertools;
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UniformsResponse {
    pub copyright: Copyright,
    #[serde(rename = "uniforms")] pub teams: Vec<TeamUniformAssets>,
}

integer_id!(UniformAssetId);
integer_id!(UniformAssetCategoryId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamUniformAssets {
    pub team_id: TeamId,
    pub uniform_assets: Vec<UniformAsset>,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum UniformAsset {
    Hydrated(HydratedUniformAsset),
    Identifiable(IdentifiableUniformAsset),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableUniformAsset {
    #[serde(rename = "uniformAssetId")] pub id: UniformAssetId,
    #[serde(rename = "uniformAssetCode")] pub code: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedUniformAsset {
    #[serde(rename = "uniformAssetText")] pub name: String,
    #[serde(rename = "uniformAssetType")] pub category: UniformAssetCategory,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableUniformAsset,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UniformAssetCategory {
    #[serde(rename = "uniformAssetTypeText")] pub name: String,
    #[serde(rename = "uniformAssetTypeCode")] pub code: String,
    #[serde(rename = "uniformAssetTypeDesc")] pub description: String,
    #[serde(rename = "uniformAssetTypeId")] pub id: UniformAssetCategoryId,
}

id_only_eq_impl!(UniformAsset, id);

#[derive(Builder)]
#[builder(derive(Into))]
pub struct UniformsRequest {
    teams: Vec<TeamId>,
    #[builder(into)]
    season: Option<SeasonId>,
}

impl<S: uniforms_request_builder::State + uniforms_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for UniformsRequestBuilder<S> {
    type Built = UniformsRequest;
}

impl Display for UniformsRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/uniforms/team{}", gen_params! { "teamIds": self.teams.iter().copied().join(","), "season"?: self.season })
    }
}

impl StatsAPIRequestUrl for UniformsRequest {
    type Response = UniformsResponse;
}

static CACHE: RwLock<HydratedCacheTable<UniformAsset>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for UniformAsset {
    type HydratedVariant = HydratedUniformAsset;
    type Identifier = String;
    type URL = UniformsRequest;

    fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
        self.try_into_hydrated()
    }

    fn id(&self) -> &Self::Identifier {
        &self.code
    }

    fn url_for_id(id: &Self::Identifier) -> Self::URL {
        UniformsRequest::builder()
            .teams(vec![TeamId::new(id.split_once('_').and_then(|(num, _)| num.parse().ok()).unwrap_or(0))])
            .build()
    }

    fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
    where
        Self: Sized
    {
        response.teams.into_iter().flat_map(|team| team.uniform_assets)
    }

    fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
    where
        Self: Sized
    {
        &CACHE
    }
}

#[cfg(test)]
mod tests {
    use crate::requests::team::uniforms::UniformsRequest;
    use crate::requests::team::teams::TeamsRequest;
    use crate::request::StatsAPIRequestUrlBuilderExt;

    #[tokio::test]
    async fn parse_all_mlb_teams_this_season() {
        let mlb_teams = TeamsRequest::mlb_teams().build_and_get().await.unwrap();
        let team_ids = mlb_teams.teams.into_iter().map(|team| team.id).collect::<Vec<_>>();
        let _ = UniformsRequest::builder().teams(team_ids).build_and_get().await.unwrap();
    }
}

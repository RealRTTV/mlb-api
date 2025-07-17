use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, Display, From};
use itertools::Itertools;
use serde::Deserialize;
use strum::EnumTryAs;
use crate::endpoints::StatsAPIUrl;
use crate::endpoints::teams::team::TeamId;
use crate::{gen_params, rwlock_const_new, RwLock};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UniformsResponse {
    pub copyright: Copyright,
    #[serde(rename = "uniforms")] pub teams: Vec<TeamUniformAssets>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamUniformAssets {
    pub team_id: TeamId,
    pub uniform_assets: Vec<UniformAsset>,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum UniformAsset {
    Hydrated(HydratedUniformAsset),
    Identifiable(IdentifiableUniformAsset),
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash)]
pub struct UniformAssetId(u32);

impl UniformAssetId {
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
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

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash)]
pub struct UniformAssetCategoryId(u32);

impl UniformAssetCategoryId {
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
}

impl PartialEq for UniformAsset {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Deref for UniformAsset {
    type Target = IdentifiableUniformAsset;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for UniformAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

pub struct UniformsEndpointUrl {
    pub teams: Vec<TeamId>,
    pub season: Option<u16>,
}

impl Display for UniformsEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/uniforms/team{}", gen_params! { "teamIds": self.teams.iter().copied().join(","), "season"?: self.season })
    }
}

impl StatsAPIUrl for UniformsEndpointUrl {
    type Response = UniformsResponse;
}

static CACHE: RwLock<HydratedCacheTable<UniformAsset>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for UniformAsset {
    type HydratedVariant = HydratedUniformAsset;
    type Identifier = String;
    type URL = UniformsEndpointUrl;

    fn into_hydrated_entry(self) -> Option<Self::HydratedVariant> {
        self.try_as_hydrated()
    }

    fn id(&self) -> &Self::Identifier {
        &self.code
    }

    fn url_for_id(id: &Self::Identifier) -> Self::URL {
        UniformsEndpointUrl {
            teams: vec![TeamId::new(id.split_once('_').and_then(|(num, _)| num.parse().ok()).unwrap_or(0))],
            season: None,
        }
    }

    fn get_entries(response: <Self::URL as StatsAPIUrl>::Response) -> impl IntoIterator<Item=Self>
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
    use crate::endpoints::sports::SportId;
    use crate::endpoints::StatsAPIUrl;
    use crate::endpoints::teams::team::uniforms::UniformsEndpointUrl;
    use crate::endpoints::teams::TeamsEndpointUrl;

    #[tokio::test]
    async fn parse_all_mlb_teams_this_season() {
        let mlb_teams = TeamsEndpointUrl { sport_id: Some(SportId::MLB), season: None }.get().await.unwrap();
        let team_ids = mlb_teams.teams.into_iter().map(|team| team.id).collect::<Vec<_>>();
        for _ in (UniformsEndpointUrl { teams: team_ids, season: None }.get().await.unwrap().teams.into_iter().flat_map(|x| x.uniform_assets).map(|x| x.try_as_hydrated().unwrap())) {}
    }
}

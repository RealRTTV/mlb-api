use serde::Deserialize;
use crate::endpoints::seasons::season::Season;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonsResponse {
    pub copyright: Copyright,
    pub seasons: Vec<Season>,
}

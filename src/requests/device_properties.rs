//! Represents CSS & HTML data for rendering teams on sites.
//!
//! Very lightly parsed due to having 0 clue what this means.
//!
//! This information is left lightly parsed as general usage of this endpoint isn't clear yet and design decisions regarding it have not been made.
//!
//! There is no known `deviceProperties` endpoint, however through some [`Hydrations`](crate::hydrations::Hydrations), this can be accessed per team or sport.

use chrono::NaiveDateTime;
use serde::Deserialize;

/// The greater struct defining device properties, see [the module docs](self)
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperties {
	#[serde(rename = "teamNameDisplay")]
	pub displayed_team_name: String,

	/// Zero clue what this means
	pub body_background_skin_wired_url: String,
	/// Zero clue what this means
	pub body_background_skin_total: usize,

	pub favicon: Asset,
	pub body_background_skin1: Asset,
	pub header_masthead_tagline: Asset,
	pub header_masthead_tagline_2x: Asset,
	pub navigation_masthead_sponsor_image: Asset,
	pub navigation_masthead_sponsor_image_2x: Asset,
	pub organism_headline_font: Asset,
	pub style: Style,
}

id!(AssetId { id: u32 });

/// An asset (?)
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Asset {
	ShortContent {
		id: AssetId,
		timestamp: NaiveDateTime,
		title: String,
		description: String,
		url: String,
		image: ImageData,
	},
	BinaryAsset {
		id: AssetId,
		timestamp: NaiveDateTime,
		#[serde(rename = "binaryFile")]
		binary_file_url: String,
	},
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
	pub title: Option<String>,
	pub alt_text: Option<String>,
	pub cuts: Option<Cuts>,
}

impl Asset {
	#[must_use]
	pub fn id(&self) -> AssetId {
		match self {
			Asset::ShortContent { id, .. } => *id,
			Asset::BinaryAsset { id, .. } => *id,
		}
	}
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Style {

}

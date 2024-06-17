use serde_derive::{Deserialize, Serialize};
use crate::models::{Image, Links, Rating};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub season: i64,
    pub number: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub airdate: String,
    pub airtime: String,
    pub airstamp: String,
    pub runtime: i64,
    pub rating: Rating,
    pub image: Image,
    pub summary: String,
    #[serde(rename = "_links")]
    pub links: Links,
}
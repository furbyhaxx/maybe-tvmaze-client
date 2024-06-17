use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use crate::{Client, Error};
use crate::models::episode::Episode;
use crate::models::season::Season;
use crate::models::{Externals, Image, Links, Network, Rating, Schedule, WebChannel};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub id: i64,
    pub url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: String,
    pub genres: Vec<String>,
    pub status: String,
    pub runtime: Option<i64>,
    pub average_runtime: Option<i64>,
    pub premiered: Option<String>,
    pub ended: Value,
    pub official_site: Option<String>,
    pub schedule: Schedule,
    pub rating: Rating,
    pub weight: i64,
    pub network: Option<Network>,
    pub web_channel: Option<WebChannel>,
    pub dvd_country: Value,
    pub externals: Externals,
    pub image: Image,
    pub summary: String,
    pub updated: i64,
    #[serde(rename = "_links")]
    pub links: Links,
    
    pub seasons: Option<Vec<Season>>,
}

impl Show {
    #[maybe_async::maybe_async]
    pub async fn get_seasons(&self) -> Result<Vec<Season>, Error> {
        Ok(Client::default().get_seasons(self.id).await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_episodes(&self) -> Result<Vec<Episode>, Error> {
        Ok(Client::default().get_episodes(self.id).await?)
    }
}
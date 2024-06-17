use serde_derive::{Deserialize, Serialize};
use crate::{Client, Error};
use crate::models::episode::Episode;
use crate::models::{Image, Links, Network, WebChannel};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonOld {
    pub number: u32,
    #[serde(rename = "episodeOrder")]
    pub episode_order: Option<u32>,
    #[serde(rename = "premiereDate")]
    pub premiere_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub id: i64,
    pub url: String,
    pub number: i64,
    pub name: String,
    pub episode_order: Option<i64>,
    pub premiere_date: String,
    pub end_date: String,
    pub network: Network,
    pub web_channel: Option<WebChannel>,
    pub image: Image,
    pub summary: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
}

impl Season {
    #[maybe_async::maybe_async]
    pub async fn get_episodes(&self) -> Result<Vec<Episode>, Error> {
        Ok(Client::default().get_season_episodes(self.id).await?)
    }
}
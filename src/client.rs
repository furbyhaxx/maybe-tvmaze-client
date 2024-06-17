use maybe_http_client::{HttpClient, Query};
use crate::Error;
use crate::models::Episode;
use crate::models::Season;
use crate::models::Show;
use crate::models::{SearchResult};
use crate::models::ShowId;


const SHOW_SEARCH_URL: &str = "https://api.tvmaze.com/search/shows";
const SHOW_SEARCH_SINGLE_URL: &str = "https://api.tvmaze.com/singlesearch/shows";
const SHOW_LOOKUP_URL: &str = "https://api.tvmaze.com/lookup/shows?";
const SHOW_INFORMATION_URL: &str = "https://api.tvmaze.com/shows/";
const SEASONS_LIST_URL: &str = "https://api.tvmaze.com/shows/%ID%/seasons";
const EPISODE_INFORMATION_URL: &str = "https://api.tvmaze.com/shows/%ID%/episodebynumber";
// const EPISODE_INFORMATION_URL: &str = "https://api.tvmaze.com/shows/%ID%/episodebynumber?season=SEASON&number=EPISODE";
const EPISODE_LIST_URL: &str = "https://api.tvmaze.com/shows/%ID%/episodes";

const SEASONS_EPISODE_LIST_URL: &str = "https://api.tvmaze.com/seasons/%ID%/episodes";

#[derive(Debug, Clone, Default)]
pub struct Client {
    http: HttpClient,
}

impl Client {
    #[maybe_async::maybe_async]
    pub async fn search(&self, q: impl AsRef<str>) -> Result<Vec<SearchResult>, Error> {

        let mut query = Query::new();
        query.insert("q".to_string(), q.as_ref().to_string());

        let result = self.http.get(
            SHOW_SEARCH_URL,
            None,
            &query,
        ).await?;

        Ok(serde_json::from_str(&result)?)
    }

    #[maybe_async::maybe_async]
    pub async fn search_best_match(&self, q: impl AsRef<str>) -> Result<Option<Show>, Error> {

        let mut query = Query::new();
        query.insert("q".to_string(), q.as_ref().to_string());

        let result = self.http.get(
            SHOW_SEARCH_SINGLE_URL,
            None,
            &query,
        ).await?;

        Ok(serde_json::from_str(&result)?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_by_id(&self, id: ShowId) -> Result<Show, Error> {
        let mut query = Query::new();

        let url = match id {
            ShowId::TvMaze(id) => {
                format!("{}{}", SHOW_INFORMATION_URL, id)
            },
            ShowId::Tvdb(id) => {
                query.insert("thetvdb".to_string(), id.to_string());
                SHOW_LOOKUP_URL.to_string()
            },
            ShowId::Imdb(id) => {
                query.insert("imdb".to_string(), id);
                SHOW_LOOKUP_URL.to_string()
            }
        };

        let result = self.http.get(
            url,
            None,
            &query,
        ).await?;

        Ok(serde_json::from_str(&result)?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_seasons(&self, id: i64) -> Result<Vec<Season>, Error> {
        let mut query = Query::new();

        let id = id.to_string();

        let result = self.http.get(
            SEASONS_LIST_URL.replace("%ID%", &id),
            None,
            &query,
        ).await?;

        Ok(serde_json::from_str(&result)?)
    }
    #[maybe_async::maybe_async]
    pub async fn get_episodes(&self, id: i64) -> Result<Vec<Episode>, Error> {
        let mut query = Query::new();

        let id = id.to_string();

        let result = self.http.get(
            EPISODE_LIST_URL.replace("%ID%", &id),
            None,
            &query,
        ).await?;

        Ok(serde_json::from_str(&result)?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_season_episodes(&self, id: i64) -> Result<Vec<Episode>, Error> {
        let mut query = Query::new();

        let id = id.to_string();

        let result = self.http.get(
            SEASONS_EPISODE_LIST_URL.replace("%ID%", &id),
            None,
            &query,
        ).await?;

        Ok(serde_json::from_str(&result)?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_episode_by_season_and_index(&self, show_id: i64, season: u64, index: u64) -> Result<Episode, Error> {
        let mut query = Query::new();

        let id = show_id.to_string();

        query.insert("season".to_string(), season.to_string());
        query.insert("number".to_string(), index.to_string());

        let result = self.http.get(
            EPISODE_INFORMATION_URL.replace("%ID%", &id),
            None,
            &query,
        ).await?;

        Ok(serde_json::from_str(&result)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use crate::models::SearchResult;
    use crate::models::ShowId;

    #[test]
    fn test_de() {
        let input = r#"  {
    "score": 1.2043885,
    "show": {
      "id": 1505,
      "url": "https://www.tvmaze.com/shows/1505/one-piece",
      "name": "One Piece",
      "type": "Animation",
      "language": "Japanese",
      "genres": [
        "Action",
        "Adventure",
        "Anime",
        "Fantasy"
      ],
      "status": "Running",
      "runtime": 25,
      "averageRuntime": 26,
      "premiered": "1999-10-20",
      "ended": null,
      "officialSite": "http://www.toei-anim.co.jp/tv/onep/",
      "schedule": {
        "time": "09:30",
        "days": [
          "Sunday"
        ]
      },
      "rating": {
        "average": 8.9
      },
      "weight": 98,
      "network": {
        "id": 131,
        "name": "Fuji TV",
        "country": {
          "name": "Japan",
          "code": "JP",
          "timezone": "Asia/Tokyo"
        },
        "officialSite": null
      },
      "webChannel": null,
      "dvdCountry": null,
      "externals": {
        "tvrage": 8205,
        "thetvdb": 81797,
        "imdb": "tt0388629"
      },
      "image": {
        "medium": "https://static.tvmaze.com/uploads/images/medium_portrait/504/1262497.jpg",
        "original": "https://static.tvmaze.com/uploads/images/original_untouched/504/1262497.jpg"
      },
      "summary": "<p><b>One Piece</b> animation is based on the successful comic by Eiichiro Oda. The comic has sold more than 260 million copies. The success doesn't stop; the <i>One Piece</i> animation series is in its top 5 TV ratings for kids programs in Japan for past few years and the series' most recent feature film title <i>\"One Piece Film Z\" </i>which was released on December 2012 has gathered 5.6 million viewers so far. The success goes beyond borders; receives high popularity on animation at terrestrial channel in Taiwan, no.1 rating on animation at a DTT channel in France, received high popularity among age 3-13 on a terrestrial channel in Germany in year 2010. The animation series has been broadcasted in many parts of the world: USA, UK, Australia, France, Spain, Portugal, Germany, Italy, Greece, Turkey, Israel, South Korea, Taiwan, China, Hong Kong, Philippine, Thailand, Singapore, Malaysia, Indonesia, Ecuador, Nicaragua, Chile, Mexico, Brazil, and etc.</p>",
      "updated": 1718120096,
      "_links": {
        "self": {
          "href": "https://api.tvmaze.com/shows/1505"
        },
        "previousepisode": {
          "href": "https://api.tvmaze.com/episodes/2899684",
          "name": "Incomprehensible! The Seraphim's Rebellion!"
        }
      }
    }
  }"#;

        let output = serde_json::from_str::<SearchResult>(&input);

        println!();
    }

    #[maybe_async::test(
        feature="sync",
        async(all(not(feature="sync"), feature="async"), async_std::test),
    )]
    async fn test_nn_client() {
        let mut client = Client::default();

        // let res = client.search("One Piece").await;
        //
        // if let Ok(data) = res {
        //     // println!("{data:?}");
        // } else {
        //     eprintln!("{}", res.unwrap_err())
        // }
        //
        // let res = client.get_by_id(ShowId::TvMaze(1505)).await;
        // let res = client.get_by_id(ShowId::Imdb("tt1119644".to_string())).await;
        // let res = client.get_seasons(1505).await;
        // let res = client.get_episodes(1505).await;
        let res = client.get_episode_by_season_and_index(1505, 1, 1).await;

        // println!("E: {:?}", res);

        println!("");
    }
}
use std::fmt::Debug;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
const BASE_URL: &str = "https://tatoeba.org/en/api_v0";

pub struct Client;

impl Client {
    pub async fn search(options: SearchOptions) -> Result<SearchResult> {
        reqwest::get(format!(
            "{BASE_URL}/search?{}",
            serde_url_params::to_string(&options).map_err(|e| anyhow!(e))?
        ))
        .await
        .map_err(|e| anyhow!(e))?
        .json::<SearchResult>()
        .await
        .map_err(|e| anyhow!(e))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sentence {
    pub id: usize,
    pub text: String,
    #[serde(rename = "lang")]
    pub language: String,
    // pub correctness: usize,
    // pub script: String,
    // pub transcriptions: Vec
    // pub audios: Vec
    pub translations: Vec<Vec<Translation>>,
    pub lang_name: String,
    #[serde(rename = "dir")]
    pub direction: String,
    // pub lang_tag: ?,
    // pub is_favorite: ?,
    pub is_owned_by_current_user: bool,
    // permissions: ?,
    pub max_visible_translations: usize,
    // current_user_review: ?,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
    pub id: usize,
    pub text: String,
    #[serde(rename = "lang")]
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub results: Vec<Sentence>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchOptions {
    #[serde(rename = "from")]
    pub language: String,
    #[serde(rename = "to")]
    pub translated_language: Option<String>,
    pub has_audio: Option<bool>,
    pub query: String,
    pub trans_filter: String,
    // pub sort: Enum
    // pub sort_reverse
}

impl SearchOptions {
    pub fn new(query: &str, language: impl ToString) -> Self {
        Self {
            language: language.to_string(),
            translated_language: Some("eng".to_string()),
            query: query.to_string(),
            has_audio: None,
            trans_filter: "limit".to_string(),
        }
    }

    pub fn translated_language(mut self, language: impl ToString) -> Self {
        self.translated_language = Some(language.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let options = SearchOptions::new("员", "chm").translated_language("eng");
        dbg!(serde_url_params::to_string(&options).unwrap());
        let result = Client::search(options).await;
        dbg!(&result);
    }
}

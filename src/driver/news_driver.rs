use crate::error::Error;
use serde::{Deserialize, Serialize};

pub async fn get_news() -> Result<NewsJson, Error> {
    let response = reqwest::get("https://hacker-news.firebaseio.com/v0/item/22018335.json")
        .await.unwrap()
        .json::<NewsJson>()
        .await.unwrap();
    Ok(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsJson {
    by: String,
    pub id: u32,
    pub parent: u32,
    pub text: String,
    time: u32,
}

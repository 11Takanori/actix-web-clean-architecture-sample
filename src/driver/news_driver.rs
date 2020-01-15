use crate::domain::news::NewsId;
use crate::error::Error;
use actix_web::client::Client;
use serde::{Deserialize, Serialize};

pub async fn get_news(id: u32) -> Result<NewsJson, Error> {
    let json = Client::default()
        .get(format!(
            "https://hacker-news.firebaseio.com/v0/item/{}/.json",
            id
        ))
        .send()
        .await?
        .json::<NewsJson>()
        .await?;
    Ok(json)
}

pub async fn get_news_ids() -> Result<NewsIdsJson, Error> {
    let json = Client::default()
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await?
        .json::<NewsIdsJson>()
        .await?;
    Ok(json)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsJson {
    by: String,
    pub id: u32,
    pub parent: Option<u32>,
    pub text: Option<String>,
    time: u32,
}

type NewsIdsJson = Vec<u32>;

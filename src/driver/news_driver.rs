use crate::error::Error;
use actix_web::client::Client;
use serde::{Deserialize, Serialize};

pub async fn get_news() -> Result<NewsJson, Error> {
    let json = Client::default()
        .get("https://hacker-news.firebaseio.com/v0/item/22018335.json")
        .send()
        .await?
        .json::<NewsJson>()
        .await?;
    Ok(json)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsJson {
    by: String,
    pub id: u32,
    pub parent: u32,
    pub text: String,
    time: u32,
}

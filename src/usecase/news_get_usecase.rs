use crate::domain::news::NewsList;
use crate::error::Error;
use crate::port::news_port::NewsPort;

pub async fn execute(news_port: impl NewsPort) -> Result<NewsList, Error> {
    let news_ids = news_port.find_news_ids().await;
    match news_ids {
        Ok(ids) => news_port.find_news(ids.into_iter().take(5).collect()).await,
        Err(e) => Err(e),
    }
}

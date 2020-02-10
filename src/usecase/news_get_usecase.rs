use crate::domain::news::NewsList;
use crate::error::Error;
use crate::port::news_port::NewsPort;

pub async fn execute(news_port: impl NewsPort) -> Result<NewsList, Error> {
    let news_ids = news_port.find_news_ids().await?;
    news_port
        .find_news(news_ids.into_iter().take(5).collect())
        .await
}

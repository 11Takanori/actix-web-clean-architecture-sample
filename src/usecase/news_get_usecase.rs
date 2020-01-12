use crate::domain::news::News;
use crate::error::Error;
use crate::port::news_port::NewsPort;

pub async fn execute(news_port: impl NewsPort) -> Result<News, Error> {
    news_port.find_news().await
}

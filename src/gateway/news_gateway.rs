use crate::domain::news::{News, NewsId};
use crate::driver::news_driver;
use crate::error::Error;
use crate::port::news_port::NewsPort;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct NewsGateway;

#[async_trait(?Send)]
impl NewsPort for NewsGateway {
    async fn find_news(self) -> Result<News, Error> {
        let news_json = news_driver::get_news().await;
        match news_json {
            Ok(news) => Ok(News::new(NewsId(news.id), NewsId(news.parent), news.text)),
            Err(e) => Err(e),
        }
    }
}

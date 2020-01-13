use crate::domain::news::{NewsIds, NewsList};
use crate::error::Error;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait NewsPort {
    async fn find_news(self, ids: NewsIds) -> Result<NewsList, Error>;
    async fn find_news_ids(self) -> Result<NewsIds, Error>;
}

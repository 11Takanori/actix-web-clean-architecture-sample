use crate::domain::news::News;
use crate::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait NewsPort {
    async fn find_news(self) -> Result<News, Error>;
}

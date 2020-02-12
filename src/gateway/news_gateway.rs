use crate::domain::news::{News, NewsId, NewsIds, NewsList};
use crate::driver::news_driver;
use crate::error::Error;
use crate::port::news_port::NewsPort;
use async_trait::async_trait;

#[derive(Debug, Copy, Clone)]
pub struct NewsGateway;

#[async_trait(?Send)]
impl NewsPort for NewsGateway {
    async fn find_news(&self, ids: NewsIds) -> Result<NewsList, Error> {
        let mut json = vec![];

        // TODO: call news_driver::get_news asynchronously
        for id in ids {
            let news_json = news_driver::get_news(id.0).await;
            json.push(news_json)
        }

        Ok(json
            .into_iter()
            .filter_map(Result::ok)
            .map(|n| {
                News::new(
                    NewsId(n.id),
                    NewsId(n.parent.unwrap_or(0)),
                    n.text.unwrap_or("".to_string()),
                )
            })
            .collect::<NewsList>())
    }

    async fn find_news_ids(&self) -> Result<NewsIds, Error> {
        let news_id_json = news_driver::get_news_ids().await;
        match news_id_json {
            Ok(ids) => Ok(ids
                .into_iter()
                .map(|id| NewsId(id))
                .collect::<Vec<NewsId>>()),
            Err(e) => Err(e),
        }
    }
}

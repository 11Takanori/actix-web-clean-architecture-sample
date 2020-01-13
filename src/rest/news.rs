use crate::domain::news::News;
use crate::rest::Container;
use crate::usecase::news_get_usecase;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub async fn news(data: web::Data<Container>) -> impl Responder {
    let news = news_get_usecase::execute(data.news_port.clone()).await;

    match news {
        Ok(news) => HttpResponse::Ok().json(
            news.into_iter()
                .map(|news| NewsJson::from(news))
                .collect::<Vec<NewsJson>>(),
        ),
        // FIXME: error handling
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewsJson {
    id: u32,
    parent_id: u32,
    text: String,
}

impl NewsJson {
    fn from(news: News) -> Self {
        NewsJson {
            id: news.id.0,
            parent_id: news.parent_id.0,
            text: news.text,
        }
    }
}

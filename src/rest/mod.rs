use crate::gateway::news_gateway::NewsGateway;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

pub mod news;
pub mod systems;

#[actix_rt::main]
pub async fn build() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Container {
                news_port: NewsGateway {},
            })
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/v1/systems/ping").route(web::get().to(systems::ping)))
        .service(web::resource("v1/news").route(web::get().to(news::news)));
}

pub struct Container {
    news_port: NewsGateway,
}

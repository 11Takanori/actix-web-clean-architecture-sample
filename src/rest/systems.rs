use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "pong": "ok"
    }))
}

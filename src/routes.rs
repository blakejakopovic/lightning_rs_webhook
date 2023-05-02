use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
pub async fn health_handler() -> impl Responder {
    let response_json = json!({
        "status": "success".to_string(),
        "message": "Ok".to_string(),
    });
    HttpResponse::Ok().json(response_json)
}

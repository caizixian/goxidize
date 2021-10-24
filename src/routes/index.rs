use actix_web::{get, web, HttpResponse};
use actix_web::http::header::LOCATION;

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/{path}")]
async fn redirect() -> HttpResponse {
    HttpResponse::Found().append_header((LOCATION, "/")).finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check).service(redirect);
}

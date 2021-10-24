use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/{path}")]
async fn redirect() -> HttpResponse {
    HttpResponse::Found()
        .append_header((LOCATION, "/"))
        .finish()
}

pub fn config(cfg: &mut web::ServiceConfig, debug: bool) {
    cfg.service(health_check).service(redirect);
    if !debug {
        cfg.route(
            "/",
            web::get().to(|| {
                HttpResponse::MovedPermanently()
                    .append_header((LOCATION, "/ui/"))
                    .finish()
            }),
        );
    }
}

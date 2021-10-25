use crate::models::Link;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use sqlx::error::Error::RowNotFound;
use sqlx::PgPool;

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/{path}")]
async fn redirect(params: web::Path<(String,)>, pg: web::Data<PgPool>) -> HttpResponse {
    let (path,) = params.into_inner();
    let row = Link::fetch_by_path(&path, pg.get_ref()).await;
    if let Err(e) = row {
        match e {
            RowNotFound => return HttpResponse::NotFound().finish(),
            _ => {
                error!("GET /{} {:?}", path, e);
            }
        }
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Found()
        .append_header((LOCATION, row.unwrap().destination))
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

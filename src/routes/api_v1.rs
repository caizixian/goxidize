use crate::models::{Link, LinkFormData};
use actix_web::{post, get, web, HttpResponse, Responder};
use sqlx::PgPool;
use serde_json::json;

#[post("/link")]
async fn new_link(link: web::Json<LinkFormData>, pg: web::Data<PgPool>) -> impl Responder {
    let link = Link::from_form_data(link.into_inner());
    let row = link.upsert(pg.get_ref()).await;
    if let Err(e) = row {
        error!("POST /link {:?} {:?}", link, e);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[get("/link")]
async fn get_links(pg: web::Data<PgPool>) -> impl Responder {
    let links = Link::fetch_all(pg.get_ref()).await;
    if let Err(e) = links {
        error!("GET /link {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().json(json!(links.unwrap()))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_link).service(get_links);
}

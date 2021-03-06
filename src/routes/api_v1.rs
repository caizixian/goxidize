use crate::models::{Link, LinkFormData};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

#[post("/link")]
#[instrument(
    skip(link, pg),
    fields(
        path = %link.path,
        destination = %link.destination
    )
)]
#[allow(clippy::async_yields_async)]
async fn new_link(link: web::Json<LinkFormData>, pg: web::Data<PgPool>) -> impl Responder {
    let link = Link::from_form_data(link.into_inner());
    let row = link.upsert(pg.get_ref()).await;
    if let Err(e) = row {
        error!("Failed to upsert a link {:?}", e);
        HttpResponse::InternalServerError().finish()
    } else {
        info!("Successfully added a link");
        HttpResponse::Ok().finish()
    }
}

#[get("/link")]
#[instrument(skip(pg))]
#[allow(clippy::async_yields_async)]
async fn get_links(pg: web::Data<PgPool>) -> impl Responder {
    let links = Link::fetch_all(pg.get_ref()).await;
    if let Err(e) = links {
        error!("Failed to fetch links {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().json(json!(links.unwrap()))
}

#[delete("/link/{path}")]
#[instrument(
    skip(pg, params),
    fields(
        path = %params.0
    )
)]
#[allow(clippy::async_yields_async)]
async fn delete_link(pg: web::Data<PgPool>, params: web::Path<(String,)>) -> impl Responder {
    let (path,) = params.into_inner();
    let result = Link::delete_by_path(&path, pg.get_ref()).await;
    if let Err(e) = result {
        error!("Failed to delete a link {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }
    if result.unwrap().rows_affected() == 0 {
        info!("Attempted to delete a link, but not found");
        HttpResponse::NotFound().finish()
    } else {
        info!("Successfully deleted a link");
        HttpResponse::Ok().finish()
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_link)
        .service(get_links)
        .service(delete_link);
}

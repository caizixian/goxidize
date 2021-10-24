use crate::models::{Link, LinkFormData};
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/link")]
async fn new_link(link: web::Json<LinkFormData>, pg: web::Data<PgPool>) -> impl Responder {
    let row = Link::from_form_data(link.into_inner())
        .upsert(pg.get_ref())
        .await;
    if let Err(e) = row {
        println!("{:?}", e);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_link);
}

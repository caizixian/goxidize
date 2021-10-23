use crate::models::{Link, LinkFormData};
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;

pub async fn index(handlebars: web::Data<Handlebars<'_>>, pg: web::Data<PgPool>) -> impl Responder {
    let links = Link::fetch_all(pg.get_ref()).await;
    if let Err(e) = links {
        println!("/[GET] {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }
    let body = handlebars
        .render(
            "index",
            &json!({
                "links": links.unwrap()
            }),
        )
        .unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn index_post(form: web::Form<LinkFormData>, pg: web::Data<PgPool>) -> impl Responder {
    let link = Link::from_form_data(form.into_inner());
    match link.upsert(pg.get_ref()).await {
        Ok(_) => HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish(),
        Err(e) => {
            println!("/[POST] {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

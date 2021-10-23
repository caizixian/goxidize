use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub async fn index(handlebars: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body = handlebars.render("index", &json!({})).unwrap();
    HttpResponse::Ok().body(body)
}

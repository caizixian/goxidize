use crate::routes::*;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use actix_files::Files;
use handlebars::Handlebars;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_templates_directory(".html", "./static/templates").unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .service(Files::new("/static", "static").prefer_utf8(true))
            .app_data(db_pool.clone())
            .app_data(handlebars_ref.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

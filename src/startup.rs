use crate::routes::*;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_templates_directory(".hbs", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/", web::post().to(index_post))
            .route("/health_check", web::get().to(health_check))
            .service(Files::new("/static", "static").prefer_utf8(true))
            .app_data(db_pool.clone())
            .app_data(handlebars_ref.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

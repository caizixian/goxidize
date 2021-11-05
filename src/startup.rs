use crate::routes::*;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use tracing_actix_web::TracingLogger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool, debug: bool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(TracingLogger::default())
            .service(web::scope("/api/v1").configure(api_v1_config))
            .configure(index_config)
            .configure(|cfg| {
                if !debug {
                    cfg.service(Files::new("/static", "./dist/"));
                    cfg.service(Files::new("/", "./dist/").index_file("index.html"));
                }
            })
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

use goxidize::configuration::CONFIGURATION;
use goxidize::startup::run;
use goxidize::telemetry::{get_otlp_tracer, get_subscriber, init_tracing};
use lazy_static::lazy_static;
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;

lazy_static! {
    static ref TRACING: () = {
        let subscriber =
            get_subscriber(&*CONFIGURATION).with(tracing_subscriber::fmt::Layer::default());
        if let Some(tracer) = get_otlp_tracer(&*CONFIGURATION) {
            let subscriber = subscriber.with(tracing_opentelemetry::layer().with_tracer(tracer));
            init_tracing(subscriber);
        } else {
            init_tracing(subscriber);
        }
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&TRACING);
    let connection_pool = PgPool::connect(&CONFIGURATION.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    let address = format!("{}:{}", CONFIGURATION.host, CONFIGURATION.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, CONFIGURATION.debug)?.await?;
    Ok(())
}

use goxidize::configuration::CONFIGURATION;
use goxidize::startup::run;
use goxidize::telemetry::{get_otlp_tracer, get_subscriber, init_tracing};
use lazy_static::lazy_static;
use sqlx::postgres::PgPool;
use sqlx::ConnectOptions;
use std::net::TcpListener;
use tracing::log::LevelFilter;
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&TRACING);
    let mut db_options = CONFIGURATION.database.options();
    db_options.log_statements(LevelFilter::Debug);
    let connection_pool = PgPool::connect_with(db_options)
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    let address = format!("{}:{}", CONFIGURATION.host, CONFIGURATION.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, CONFIGURATION.debug)?.await?;
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

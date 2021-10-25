use env_logger::Env;
use goxidize::configuration::get_configuration;
use goxidize::startup::run;
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.yml.");
    let default_logging_level = if configuration.debug { "info" } else { "warn" };
    env_logger::Builder::from_env(Env::default().default_filter_or(default_logging_level)).init();
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    let address = format!("127.0.0.1:{}", configuration.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, configuration.debug)?.await?;
    Ok(())
}

use goxide::configuration::get_configuration;
use goxide::startup::run;
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.yml.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, configuration.debug)?.await?;
    Ok(())
}

use goxidize::configuration::{DatabaseSettings, CONFIGURATION};
use goxidize::startup::run;
use goxidize::telemetry::{get_subscriber, init_tracing};
use lazy_static::lazy_static;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use uuid::Uuid;

lazy_static! {
    static ref TRACING: () = {
        let subscriber = get_subscriber(&*CONFIGURATION)
            .with(tracing_subscriber::fmt::Layer::default().with_test_writer());
        init_tracing(subscriber);
    };
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    lazy_static::initialize(&TRACING);
    let mut database_config = CONFIGURATION.database.clone();
    database_config.name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&database_config).await;

    let server = run(listener, connection_pool.clone(), CONFIGURATION.debug)
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.options_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.options())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

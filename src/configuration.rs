use sqlx::postgres::PgConnectOptions;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub otlpendpoint: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub port: String,
    pub host: String,
    pub name: String,
}

impl DatabaseSettings {
    pub fn options(&self) -> PgConnectOptions {
        self.options_without_db().database(&self.name)
    }

    pub fn options_without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port.parse().expect("Failed to parse port number"))
    }
}

lazy_static! {
    pub static ref CONFIGURATION: Settings =
        get_configuration().expect("Failed to read configuration.yml.");
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings
        .set_default("debug", false)
        .expect("Failed to set the default value for debug");
    settings
        .set_default("host", "127.0.0.1")
        .expect("Failed to set the default value for host");
    settings
        .set_default("otlpendpoint", "")
        .expect("Failed to set the default value for otlpendpoint");
    settings.merge(config::File::with_name("configuration").required(false))?;
    settings.merge(config::Environment::new().prefix("goxidize").separator("_"))?;
    settings.try_into()
}

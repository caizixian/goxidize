use sqlx::postgres::PgConnectOptions;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub otlpendpoint: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub port: String,
    pub host: String,
    pub name: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl DatabaseSettings {
    pub fn options(&self) -> PgConnectOptions {
        self.options_without_db().database(&self.name)
    }

    pub fn options_without_db(&self) -> PgConnectOptions {
        println!("{:?}", self);
        let options = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port.parse().expect("Failed to parse port number"));
        if let Some(ref username) = self.username {
            let password = self
                .password
                .as_ref()
                .expect("Password expected when a username is set");
            options.username(username).password(password)
        } else {
            options
        }
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
    settings.merge(config::File::with_name("configuration").required(false))?;
    settings.merge(config::Environment::new().prefix("goxidize").separator("_"))?;
    settings.try_into()
}

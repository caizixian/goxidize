#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("{}/{}", self.url, self.name)
    }

    pub fn connection_string_without_db(&self) -> String {
        self.url.clone()
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

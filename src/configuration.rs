#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
}

#[derive(serde::Deserialize)]
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

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration").required(false))?;
    settings.merge(config::Environment::new().prefix("goxide").separator("_"))?;
    settings.try_into()
}

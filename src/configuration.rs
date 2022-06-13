#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize the config reader
    let mut settings = config::Config::default();

    // Will look in the top level directory for a file named config
    settings.merge(config::File::with_name("configuration"))?;

    // Try converting the file into a Settings type
    settings.try_into()
}

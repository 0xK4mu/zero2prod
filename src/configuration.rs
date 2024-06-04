#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSetting,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSetting {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
    // Add configuration valpues from a file name 'configuration.yaml'
    .add_source(
        config::File::new("configuration.yaml", config::FileFormat::Yaml)
    )
    .build()?;
    // Try to convert the config values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}


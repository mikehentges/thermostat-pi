use dotenv::dotenv;
use secrecy::ExposeSecret;
use secrecy::Secret;
use std::convert::TryInto;

#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}
#[derive(Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
#[derive(Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

pub fn get_configuration() -> Result<Settings, Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_settings = DatabaseSettings {
        username: std::env::var("DB_USERNAME")?,
        password: std::env::var("DB_PASSWORD")?.try_into()?,
        port: std::env::var("DB_PORT")?.parse()?,
        host: std::env::var("DB_HOST")?,
        database_name: std::env::var("DB_NAME")?,
    };

    let app_settings = ApplicationSettings {
        port: std::env::var("APP_PORT")?.parse()?,
        host: std::env::var("APP_HOST")?,
        base_url: std::env::var("APP_BASE_URL")?,
    };

    let settings = Settings {
        database: db_settings,
        application: app_settings,
    };

    Ok(settings)
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
        ))
    }
}

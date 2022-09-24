use config::Config;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub push_lambda_url: String,
    pub initial_thermostat_value: usize,
    pub poll_interval: usize,
}
#[tracing::instrument(name = "getting the configuration")]
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;

    let app_settings: Settings = settings.try_deserialize().unwrap();

    Ok(app_settings)
}

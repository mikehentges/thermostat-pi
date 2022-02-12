#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub push_lambda_url: String,
    pub initial_thermostat_value: usize,
}
#[tracing::instrument(name = "getting the configuration")]
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}

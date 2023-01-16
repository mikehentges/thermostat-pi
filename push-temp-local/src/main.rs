use push_temp_local::configuration::get_configuration;
use push_temp_local::startup::Application;
use push_temp_local::telemetry::{get_subscriber, init_subscriber};

// This is a test
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}

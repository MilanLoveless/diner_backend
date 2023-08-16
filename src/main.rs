use diner_backend::configuration::get_configuration;
use diner_backend::startup::Application;
use diner_backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Tracing
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Config
    let configuration = get_configuration().expect("Failed to read configuration.");
    // App
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}

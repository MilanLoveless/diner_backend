use diner_backend::configuration::{get_configuration, DatabaseSettings};
use diner_backend::startup::{get_connection_pool, Application};
use diner_backend::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use serde::Serialize;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub discord_oauth: MockServer,
    pub discord_api: MockServer,
}

impl TestApp {
    pub async fn post<T>(&self, path: &str, payload: &T) -> reqwest::Response
    where
        T: Serialize + ?Sized,
    {
        reqwest::Client::new()
            .post(&format!("{}/{}", &self.address, path))
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get(&self, path: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/{}", &self.address, path))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_by_id(&self, path: &str, id: Uuid) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/{}/{}", &self.address, path, id))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_with_query<T>(&self, path: &str, query: &T) -> reqwest::Response
    where
        T: Serialize + ?Sized,
    {
        reqwest::Client::new()
            .get(&format!("{}/{}", &self.address, path))
            .header("Content-Type", "application/json")
            .query(query)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn put<T>(&self, path: &str, payload: &T, id: Uuid) -> reqwest::Response
    where
        T: Serialize + ?Sized,
    {
        reqwest::Client::new()
            .put(&format!("{}/{}/{}", &self.address, path, id))
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete(&self, path: &str, id: Uuid) -> reqwest::Response {
        reqwest::Client::new()
            .delete(&format!("{}/{}/{}", &self.address, path, id))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let discord_oauth = MockServer::start().await;
    let discord_api = MockServer::start().await;

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c.discord_oauth.uri = discord_oauth.uri();
        c.discord_api.uri = discord_api.uri();
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    // Launch the application as a background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let address = format!("http://localhost:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        discord_oauth,
        discord_api,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

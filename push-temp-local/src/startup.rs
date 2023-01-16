use crate::configuration::Settings;
use crate::routes::health_check::health_check;
use crate::routes::home::home;
use crate::routes::store_temperature::store_temperature;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(home))
            .route("/store_temperature", web::post().to(store_temperature))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool =
            get_connection_pool(configuration.database.connection_string().expose_secret());

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
        )
        .await?;

        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(connection_string: &str) -> PgPool {
    PgPool::connect_lazy(connection_string).expect("Failed to connect to db.")
}

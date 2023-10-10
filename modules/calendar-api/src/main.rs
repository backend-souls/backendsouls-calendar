use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use calendar_api::configuration::configuration;
use calendar_api::telemetry::{init_subscriber, subscriber};
use calendar_api::version::version;

use calendar_api::environment::Environment;
use serde::Serialize;
use std::net::SocketAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let subscriber = subscriber("backendsouls_calendar".into(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = configuration(environment).expect("Failed to read configuration.");

    let app = Router::new()
        .route("/:version/version", get(version))
        .route("/health", get(health));

    // run it
    let addr = SocketAddr::from_str(&format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct Message {
    code: u16,
    message: String,
}

pub async fn health() -> impl IntoResponse {
    let message = Message {
        code: 200,
        message: "Hello World!".to_string(),
    };

    (StatusCode::OK, Json(message))
}

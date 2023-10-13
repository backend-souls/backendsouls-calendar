use anyhow::Context;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

use calendar_api::app::CalendarApplication;
use calendar_api::configuration::configuration;
use calendar_api::environment::Environment;
use calendar_api::telemetry::{init_subscriber, subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let subscriber = subscriber("backendsouls_calendar".into(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = configuration(environment).expect("Failed to read configuration.");

    //    let app = Router::new()
    //        .route("/:version/version", get(version))
    //        .route("/health", get(health));

    CalendarApplication::serve(configuration.application)
        .await
        .context("could not initialize application routes")?;

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

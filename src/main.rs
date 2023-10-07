use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use rfc8984_calendar::configuration::configuration;
use rfc8984_calendar::telemetry::{init_subscriber, subscriber};
use rfc8984_calendar::version::version;
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = subscriber("rfc8984_calendar".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let configuration = configuration().expect("Failed to read configuration.");

    let app = Router::new()
        .route("/:version/version", get(version))
        .route("/health", get(health));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], configuration.application.port));

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

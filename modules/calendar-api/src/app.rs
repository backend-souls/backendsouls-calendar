use std::net::SocketAddr;
use std::str::FromStr;

use anyhow::Context;
use axum::Router;

use crate::configuration::ApplicationSettings;
use crate::event::EventsRouter;

pub struct CalendarApplication;

impl CalendarApplication {
    pub async fn serve(app_configuration: ApplicationSettings) -> anyhow::Result<()> {
        let router = Router::new().nest("/api", EventsRouter::routes());

        // run it
        let addr = SocketAddr::from_str(&format!("{}:{}", app_configuration.host, app_configuration.port))?;

        tracing::info!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("error while starting API server")?;

        Ok(())
    }
}

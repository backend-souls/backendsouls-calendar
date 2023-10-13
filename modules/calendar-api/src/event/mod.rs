use axum::extract::Path;
use axum::routing::get;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Router};
use derive_builder::Builder;
use serde_with::skip_serializing_none;

use backendsouls_json_hal::get_default_response;
use serde::Serialize;
use uuid::Uuid;

pub mod request;
pub mod response;

const MEDIA_TYPE: &str = "application/jscalendar+json;type=event";

pub struct EventsRouter;

impl EventsRouter {
    pub fn routes() -> Router {
        let f = get_event;
        Router::new().route("/events/:id", get(get_event))
        // add controller

        // add services

        // add repositories

        // database connection added
    }
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize)]
#[builder(setter(into), default)]
pub struct EventDto {
    id: u32,
    external_id: Uuid,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct GetEventResponse {}

pub async fn get_event(Path(external_id): Path<Uuid>) -> impl IntoResponse {
    tracing::info!("event with external_id {}", external_id);

    let event = EventDtoBuilder::default()
        .id(1u32)
        .external_id(external_id)
        .content("some content".to_string())
        .build()
        .unwrap();

    // since we are working with external_id setup,
    // the next event is not easily found like increasing id value
    // also, the next event could be restricted by some user group rule,
    // permission
    //
    // _self (event_0): id = 1 (user001)
    // _next (event_1): id = 2 (user002)
    // event_2: id = 3 (user001)
    //
    // this way, for the user001, the next event is event_0
    //
    // so for the next link we need to make a db query, the option should
    // configurable, enabling via api configuration or deployment configuration
    //
    // -- for now all the configurations should be via deployment
    //

    let response = get_default_response(format!("/events/{}", external_id), event);

    (StatusCode::OK, Json(response))
}

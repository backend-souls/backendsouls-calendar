use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, Json, RequestPartsExt};
use serde::Serialize;
use std::collections::HashMap;

#[tracing::instrument(name = "Testing versions")]
pub async fn version(version: Version) -> impl IntoResponse {
    match version {
        Version::V1 => (StatusCode::OK, Json(VersionStatus::Active)),
        Version::V2 => (StatusCode::OK, Json(VersionStatus::NotImplemented)),
        Version::V3 => (StatusCode::OK, Json(VersionStatus::NotImplemented)),
    }
}

#[derive(Debug)]
pub enum Version {
    V1,
    V2,
    V3,
}

#[derive(Debug, Serialize)]
pub enum VersionStatus {
    Active,
    NotImplemented,
    Deprecated,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> = parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            "v3" => Ok(Version::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}

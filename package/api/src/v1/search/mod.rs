use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;

use crate::{common::CommonQueryResponse, utils::error::ApiError};

use models::prelude::*;

pub fn router() -> Router {
    Router::new().route("/search", get(search_handler))
}
#[derive(Deserialize)]
struct SearchParams {
    query: String,
}
async fn search_handler(
    Query(params): Query<SearchParams>,
) -> Result<Json<CommonQueryResponse<SearchResult>>, ApiError> {
    if params.query.trim().is_empty() {
        tracing::error!("Query Required");
        return Err(ApiError::ValidationError("\"Query Required\"".to_string()));
    }
    Ok(Json(CommonQueryResponse {
        items: vec![SearchResult::new(
            "proof-id".to_string(),
            Some(1),
            Some(2),
            None,
            Some(Age::Specific(20)),
        )],
        bookmark: None,
    }))
}
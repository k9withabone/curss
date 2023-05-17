use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/healthcheck", get(healthcheck))
}

#[allow(clippy::unused_async)]
async fn healthcheck() -> impl IntoResponse {
    StatusCode::OK
}

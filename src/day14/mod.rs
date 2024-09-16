use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::post, Json, Router};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
struct UnsafeTemplate {
    content: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct SafeTemplate {
    content: String,
}

#[derive(Deserialize)]
struct UnsafeRequest {
    content: String,
}

async fn unsafe_route(Json(req): Json<UnsafeRequest>) -> impl IntoResponse {
    let content = req.content;
    UnsafeTemplate { content }
}

async fn safe_route(Json(req): Json<UnsafeRequest>) -> impl IntoResponse {
    let content = req.content;
    SafeTemplate { content }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/14/unsafe", post(unsafe_route))
        .route("/14/safe", post(safe_route))
}

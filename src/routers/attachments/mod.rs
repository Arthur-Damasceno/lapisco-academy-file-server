mod download;
mod information;
mod remove;
mod upload;

use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post, Router},
};

const BODY_LIMIT: usize = 1024 * 1_000_000;

pub fn router() -> Router {
    Router::new()
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .route("/:id", get(download::handle))
        .route("/:id/info", get(information::handle))
        .route("/", post(upload::handle))
        .route("/:id", delete(remove::handle))
}

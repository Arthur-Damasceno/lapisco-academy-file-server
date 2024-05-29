mod download;
mod upload;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post, Router},
};

use crate::database::Database;

const BODY_LIMIT: usize = 1024 * 1_000_000;

pub fn app(database: Database) -> Router {
    Router::new()
        .route("/", post(upload::handle))
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .route("/:id", get(download::handle))
        .with_state(database)
}

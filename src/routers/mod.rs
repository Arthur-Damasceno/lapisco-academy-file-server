mod attachments;

use axum::{Extension, Router};

use crate::database::Database;

pub fn app(database: Database) -> Router {
    Router::new()
        .nest("/attachments", attachments::router())
        .layer(Extension(database))
}

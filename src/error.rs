use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("A request has invalid data")]
    InvalidData,
    #[error("The requested resource was not found")]
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidData => StatusCode::BAD_REQUEST.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            error => {
                eprintln!("{error}");

                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

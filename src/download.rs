use axum::{body::Body, http::header, response::IntoResponse};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn handle() -> impl IntoResponse {
    let file = File::open("test.mp4").await.unwrap();
    let body = Body::from_stream(ReaderStream::new(file));
    let headers = [
        (header::CONTENT_TYPE, "video/mp4"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"test.mp4\"",
        ),
    ];

    (headers, body)
}

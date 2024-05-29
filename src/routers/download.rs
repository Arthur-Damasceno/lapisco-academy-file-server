use axum::{body::Body, extract::Path, http::header, response::IntoResponse};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn handle(Path(name): Path<String>) -> impl IntoResponse {
    let file = File::open(format!("./upload/{name}")).await.unwrap();
    let body = Body::from_stream(ReaderStream::new(file));
    let headers = [
        (header::CONTENT_TYPE, "video/mp4".into()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{name}\""),
        ),
    ];

    (headers, body)
}

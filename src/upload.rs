use axum::{http::StatusCode, extract::Multipart};
use tokio::fs::write;

pub async fn handle(mut multipart: Multipart) -> StatusCode {
    if let Ok(Some(field)) = multipart.next_field().await {
        if let Ok(data) = field.bytes().await {
            write("test.txt", &data).await.unwrap();
        }
    }

    StatusCode::CREATED
}

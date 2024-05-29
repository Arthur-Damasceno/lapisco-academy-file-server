use axum::{extract::Multipart, http::StatusCode};
use tokio::fs::write;

pub async fn handle(mut multipart: Multipart) -> StatusCode {
    if let Ok(Some(field)) = multipart.next_field().await {
        let name = field
            .file_name()
            .map_or("unknown".into(), |name| name.to_owned());

        if let Ok(data) = field.bytes().await {
            write(format!("./upload/{name}"), &data).await.unwrap();
        }
    }

    StatusCode::CREATED
}

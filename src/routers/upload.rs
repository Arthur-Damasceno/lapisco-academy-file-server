use axum::{
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
};
use tokio::fs::write;

use crate::{
    database::Database,
    models::{Attachment, AttachmentExtension},
};

pub async fn handle(
    State(db): State<Database>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Attachment>), StatusCode> {
    if let Ok(Some(field)) = multipart.next_field().await {
        let attachment = Attachment::new(AttachmentExtension::Mp4);

        db.insert_attachment(&attachment)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Ok(data) = field.bytes().await {
            save_attachment(data, &attachment)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok((StatusCode::CREATED, Json(attachment)));
        }
    }

    Err(StatusCode::BAD_REQUEST)
}

async fn save_attachment(data: Bytes, Attachment { id, extension }: &Attachment) -> crate::Result {
    let path = match extension {
        AttachmentExtension::Mp4 => format!("./upload/{id}.mp4"),
    };

    write(path, &data).await?;

    Ok(())
}

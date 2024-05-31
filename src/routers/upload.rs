use axum::{
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
};
use tokio::fs::write;

use crate::{
    database::Database,
    error::{Error, Result},
    models::{Attachment, AttachmentExtension},
};

pub async fn handle(
    State(db): State<Database>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Attachment>)> {
    let field = multipart
        .next_field()
        .await
        .map_err(|_| Error::InvalidData)
        .and_then(|field| field.ok_or_else(|| Error::InvalidData))?;

    let attachment = Attachment::new(AttachmentExtension::Mp4);

    db.insert_attachment(&attachment).await?;

    let data = field.bytes().await.map_err(|_| Error::InvalidData)?;

    save_attachment(data, &attachment).await?;

    Ok((StatusCode::CREATED, Json(attachment)))
}

async fn save_attachment(data: Bytes, Attachment { id, extension }: &Attachment) -> Result {
    let path = match extension {
        AttachmentExtension::Mp4 => format!("./upload/{id}.mp4"),
    };

    write(path, &data).await.map_err(From::from)
}

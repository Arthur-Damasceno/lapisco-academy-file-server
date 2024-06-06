use axum::{
    body::Bytes,
    extract::{Extension, Multipart},
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
    Extension(db): Extension<Database>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Attachment>)> {
    let field = multipart
        .next_field()
        .await
        .map_err(|_| Error::InvalidData)
        .and_then(|field| field.ok_or_else(|| Error::InvalidData))?;

    let extension = field
        .content_type()
        .ok_or(Error::InvalidData)
        .and_then(|content_type| AttachmentExtension::try_from(content_type))?;
    let attachment = Attachment::new(extension);

    db.insert_attachment(&attachment).await?;

    let data = field.bytes().await.map_err(|_| Error::InvalidData)?;

    save_attachment(data, &attachment).await?;

    Ok((StatusCode::CREATED, Json(attachment)))
}

async fn save_attachment(data: Bytes, attachment: &Attachment) -> Result {
    let mut path = attachment.filename();
    path.insert_str(0, "./upload/");

    write(path, &data).await.map_err(From::from)
}

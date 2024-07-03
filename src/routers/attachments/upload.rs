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
    models::Attachment,
};

pub async fn handle(
    Extension(db): Extension<Database>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Attachment>)> {
    let field = multipart
        .next_field()
        .await
        .map_err(|_| Error::InvalidData)
        .and_then(|field| field.ok_or(Error::InvalidData))?;

    let filename = field.file_name().ok_or(Error::InvalidData)?.into();
    let content_type = field.content_type().ok_or(Error::InvalidData)?.into();

    let attachment = Attachment::new(filename, content_type);

    db.insert_attachment(&attachment).await?;

    let data = field.bytes().await.map_err(|_| Error::InvalidData)?;

    save_attachment(&attachment, data).await?;

    Ok((StatusCode::CREATED, Json(attachment)))
}

async fn save_attachment(Attachment { id, .. }: &Attachment, data: Bytes) -> Result {
    let path = format!("./upload/{id}");

    write(path, &data).await.map_err(From::from)
}

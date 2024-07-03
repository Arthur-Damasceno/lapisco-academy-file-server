use axum::{
    body::Body,
    extract::{Extension, Path},
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use {tokio::fs::File, tokio_util::io::ReaderStream};

use crate::{
    database::Database,
    error::{Error, Result},
    models::Attachment,
};

pub async fn handle(
    Extension(db): Extension<Database>,
    Path(id): Path<String>,
) -> Result<Response> {
    let attachment = db.find_attachment_by_id(&id).await?;

    if let Some(Attachment {
        id,
        filename,
        content_type,
    }) = attachment
    {
        let headers = [
            (CONTENT_TYPE, content_type),
            (
                CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ];

        let body = read_attachment(&id).await?;

        return Ok((headers, body).into_response());
    }

    Err(Error::NotFound)
}

async fn read_attachment(id: &str) -> Result<Body> {
    let path = format!("./upload/{id}");
    let file = File::open(path).await?;
    let body = Body::from_stream(ReaderStream::new(file));

    Ok(body)
}

use axum::{
    body::Body,
    extract::{Path, State},
    http::{
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
        StatusCode,
    },
    response::{IntoResponse, Response},
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::database::Database;

pub async fn handle(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Response, StatusCode> {
    if let Some(attachment) = db
        .find_attachment_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let filename = attachment.filename();

        let headers = [
            (CONTENT_TYPE, attachment.extension.content_type().into()),
            (
                CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ];

        let body = read_attachment(&filename)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok((headers, body).into_response());
    }

    Err(StatusCode::NOT_FOUND)
}

async fn read_attachment(filename: &str) -> crate::Result<Body> {
    let file = File::open(format!("./upload/{filename}")).await?;
    let body = Body::from_stream(ReaderStream::new(file));

    Ok(body)
}

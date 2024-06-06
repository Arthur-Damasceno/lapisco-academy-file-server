use axum::{
    body::Body,
    extract::{Extension, Path},
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{
    database::Database,
    error::{Error, Result},
};

pub async fn handle(
    Extension(db): Extension<Database>,
    Path(id): Path<String>,
) -> Result<Response> {
    let attachment = db.find_attachment_by_id(&id).await?;

    if let Some(attachment) = attachment {
        let filename = attachment.filename();

        let headers = [
            (CONTENT_TYPE, attachment.extension.content_type().into()),
            (
                CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ];

        let body = read_attachment(&filename).await?;

        return Ok((headers, body).into_response());
    }

    Err(Error::NotFound)
}

async fn read_attachment(filename: &str) -> Result<Body> {
    let path = format!("./upload/{filename}");
    let file = File::open(path).await?;
    let body = Body::from_stream(ReaderStream::new(file));

    Ok(body)
}

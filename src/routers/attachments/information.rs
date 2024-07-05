use axum::extract::{Extension, Json, Path};

use crate::{
    database::Database,
    error::{Error, Result},
    models::Attachment,
};

pub async fn handle(
    Extension(db): Extension<Database>,
    Path(id): Path<String>,
) -> Result<Json<Attachment>> {
    let attachement = db
        .find_attachment_by_id(&id)
        .await
        .and_then(|attachment| attachment.ok_or(Error::NotFound))?;

    Ok(Json(attachement))
}

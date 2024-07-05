use axum::{
    extract::{Extension, Path},
    http::StatusCode,
};
use tokio::{fs::remove_file, task::spawn};

use crate::{
    database::Database,
    error::{Error, Result},
};

pub async fn handle(
    Extension(db): Extension<Database>,
    Path(id): Path<String>,
) -> Result<StatusCode> {
    let deleted = db.delete_attachment_by_id(&id).await?;

    if deleted {
        remove_attachment(id);

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(Error::NotFound)
    }
}

fn remove_attachment(id: String) {
    let path = format!("./upload/{id}");

    spawn(async move {
        if let Err(error) = remove_file(path).await {
            eprintln!("Error while removing file {id}: {error}");
        }
    });
}

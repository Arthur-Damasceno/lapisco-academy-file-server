use std::sync::Arc;

use sqlx::{query, SqlitePool};

use crate::{error::Result, models::Attachment};

#[derive(Clone)]
pub struct Database(Arc<SqlitePool>);

impl Database {
    pub async fn connect(url: &str) -> Result<Self> {
        let connection = SqlitePool::connect(url).await?;

        Ok(Self(connection.into()))
    }

    pub async fn find_attachment_by_id(&self, id: &str) -> Result<Option<Attachment>> {
        let mut conn = self.0.acquire().await?;

        let attachment = query!("SELECT * FROM attachments WHERE id = ?;", id)
            .fetch_optional(&mut *conn)
            .await?
            .map(|rec| Attachment {
                id: rec.id,
                extension: rec.extension.into(),
            });

        Ok(attachment)
    }

    pub async fn insert_attachment(&self, Attachment { id, extension }: &Attachment) -> Result {
        let mut conn = self.0.acquire().await?;

        let extension = *extension as u8;
        query!(
            "INSERT INTO attachments (id, extension) VALUES (?, ?);",
            id,
            extension
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

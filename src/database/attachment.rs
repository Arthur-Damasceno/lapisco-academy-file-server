use sqlx::query;

use crate::{database::Database, error::Result, models::Attachment};

impl Database {
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

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
                filename: rec.filename,
                content_type: rec.content_type,
            });

        Ok(attachment)
    }

    pub async fn insert_attachment(
        &self,
        Attachment {
            id,
            filename,
            content_type,
        }: &Attachment,
    ) -> Result {
        let mut conn = self.0.acquire().await?;

        query!(
            "INSERT INTO attachments (id, filename, content_type) VALUES (?, ?, ?);",
            id,
            filename,
            content_type
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn delete_attachment_by_id(&self, id: &str) -> Result<bool> {
        let mut conn = self.0.acquire().await?;

        let rows_affected = query!("DELETE FROM attachments WHERE id = ?;", id)
            .execute(&mut *conn)
            .await?
            .rows_affected();

        Ok(rows_affected == 1)
    }
}

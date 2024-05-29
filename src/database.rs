use std::sync::Arc;

use sqlx::{query, SqlitePool};

use crate::models::Attachment;

#[derive(Clone)]
pub struct Database(Arc<SqlitePool>);

impl Database {
    pub async fn connect(url: &str) -> crate::Result<Self> {
        let connection = SqlitePool::connect(url).await?;

        Ok(Self(connection.into()))
    }

    pub async fn insert_attachment(
        &self,
        Attachment { id, extension }: &Attachment,
    ) -> crate::Result {
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

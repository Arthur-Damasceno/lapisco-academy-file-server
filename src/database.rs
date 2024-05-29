use std::sync::Arc;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Database(Arc<SqlitePool>);

impl Database {
    pub async fn connect(url: &str) -> crate::Result<Self> {
        let connection = SqlitePool::connect(url).await?;

        Ok(Self(connection.into()))
    }
}

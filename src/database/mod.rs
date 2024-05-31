mod attachment;

use std::sync::Arc;

use sqlx::SqlitePool;

use crate::error::Result;

#[derive(Clone)]
pub struct Database(Arc<SqlitePool>);

impl Database {
    pub async fn connect(url: &str) -> Result<Self> {
        let connection = SqlitePool::connect(url).await?;

        Ok(Self(connection.into()))
    }
}

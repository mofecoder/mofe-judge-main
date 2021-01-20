use crate::db::DbPool;
use crate::models::Submit;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SubmitRepository {
    async fn get_submits(&self) -> Result<Vec<Submit>>;
}

#[async_trait]
impl SubmitRepository for DbPool {
    async fn get_submits(&self) -> Result<Vec<Submit>> {
        let submits = sqlx::query_as(
            r#"
            SELECT * FROM submits 
            WHERE status = 'WJ' OR status = 'WR' AND deleted_at IS NULL
            ORDER BY updated_at ASC
            LIMIT 2
            "#,
        )
        .fetch_all(self)
        .await?;

        Ok(submits)
    }
}

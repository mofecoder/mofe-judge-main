use anyhow::Result;
use sqlx::{mysql::MySqlPool, MySql};
pub type DbPool = sqlx::Pool<MySql>;

pub async fn new_pool(database_url: &str) -> Result<DbPool> {
    let pool = MySqlPool::connect(database_url).await?;
    Ok(pool)
}

#[cfg(feature = "db_test")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::config::ENV_CONFIG;
    #[tokio::test]
    async fn test_db() -> Result<()> {
        use crate::repository::SubmitRepository;
        let pool = new_pool(&ENV_CONFIG.database_url).await?;
        let result = pool.begin().await?.get_submits().await?;
        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_db2() -> Result<()> {
        use crate::repository::ProblemsRepository;
        let pool = new_pool(&ENV_CONFIG.database_url).await?;
        let result = pool.fetch_problem(1).await?;
        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_db3() -> Result<()> {
        use crate::repository::TestcasesRepository;
        let pool = new_pool(&ENV_CONFIG.database_url).await?;
        let result = pool.fetch_testcases(1).await?;
        dbg!(result);

        Ok(())
    }
}

use crate::config::Config;
use anyhow::Result;
use sqlx::{mysql::MySqlPool, MySql};
pub type DbPool = sqlx::Pool<MySql>;

pub async fn new_pool(config: &Config) -> Result<DbPool> {
    let pool = MySqlPool::connect(&config.database_url).await?;
    Ok(pool)
}

#[cfg(feature = "db_test")]
#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_db() -> Result<()> {
        use crate::config::load_config;
        use crate::repository::SubmitRepository;
        let config = load_config()?;
        let pool = new_pool(&config).await?;
        let result = pool.begin().await?.get_submits().await?;
        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_db2() -> Result<()> {
        use crate::config::load_config;
        use crate::repository::ProblemsRepository;
        let config = load_config()?;
        let pool = new_pool(&config).await?;
        let result = pool.fetch_problem(1).await?;
        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_db3() -> Result<()> {
        use crate::config::load_config;
        use crate::repository::TestcasesRepository;
        let config = load_config()?;
        let pool = new_pool(&config).await?;
        let result = pool.fetch_testcases(1).await?;
        dbg!(result);

        Ok(())
    }
}

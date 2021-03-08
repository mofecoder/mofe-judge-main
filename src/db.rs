use crate::config::Config;
use anyhow::Result;
use sqlx::{mysql::MySqlPool, MySql};
pub type DbPool = sqlx::Pool<MySql>;

pub async fn new_pool(config: &Config) -> Result<DbPool> {
    let pool = MySqlPool::connect(&config.database_url).await?;
    Ok(pool)
}

#[tokio::test]
async fn test_db() -> Result<()> {
    use crate::repository::SubmitRepository;
    let config = Config {
        database_url: "mysql://root:root@localhost:3306/p6jav_cafecoder_prod".to_string(),
    };
    let pool = new_pool(&config).await?;
    let result = pool.begin().await?.get_submits().await?;
    dbg!(result);

    Ok(())
}

#[tokio::test]
async fn test_db2() -> Result<()> {
    use crate::repository::ProblemsRepository;
    let config = Config {
        database_url: "mysql://root:root@localhost:3306/p6jav_cafecoder_prod".to_string(),
    };
    let pool = new_pool(&config).await?;
    let result = pool.fetch_problem(1).await?;
    dbg!(result);

    Ok(())
}

#[tokio::test]
async fn test_db3() -> Result<()> {
    use crate::repository::TestcasesRepository;
    let config = Config {
        database_url: "mysql://root:root@localhost:3306/p6jav_cafecoder_prod".to_string(),
    };
    let pool = new_pool(&config).await?;
    let result = pool.fetch_testcases(1).await?;
    dbg!(result);

    Ok(())
}

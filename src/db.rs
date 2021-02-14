use crate::config::Config;
use anyhow::Result;
use sqlx::{mysql::MySqlPool, MySql};

pub type DbPool = sqlx::Pool<MySql>;

pub async fn new_pool(config: &Config) -> Result<DbPool> {
    let pool = MySqlPool::connect(&config.database_url).await?;
    Ok(pool)
}

use dotenv::dotenv;
use sqlx::{mysql::MySqlPool, MySql};
use std::env;

pub async fn new_pool() -> Result<sqlx::Pool<MySql>, sqlx::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&url).await?;

    Ok(pool)
}

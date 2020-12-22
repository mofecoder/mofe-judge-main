use dotenv::dotenv;
use sqlx::{MySql, mysql::MySqlPool};
use std::env;

pub async fn new_pool() -> Result<sqlx::Pool<MySql>, sqlx::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut pool = MySqlPool::connect(&url).await?;

    Ok(pool)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



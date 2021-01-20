use anyhow::Result;

pub struct Config {
    pub database_url: String,
}

pub fn load_config() -> Result<Config> {
    let database_url = dotenv::var("DATABASE_URL")?;
    Ok(Config { database_url })
}

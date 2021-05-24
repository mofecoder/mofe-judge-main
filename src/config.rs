use once_cell::sync::Lazy;

pub static ENV_CONFIG: Lazy<Config> = Lazy::new(load_config);
#[derive(Clone, Default)]
pub struct Config {
    pub database_url: String,
    pub database_pool_size: u32,
    pub judge_container_port: String,
    pub docker_address: String,
}

pub fn load_config() -> Config {
    const DEFAULT_JUDGE_CONTAINER_PORT: &str = "8000";
    const DEFAULT_DOCKER_ADDRESS: &str = "http://localhost:2375";
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let database_pool_size = dotenv::var("DATABASE_POOL_SIZE")
        .unwrap()
        .parse::<u32>()
        .unwrap_or(6);
    let judge_container_port = dotenv::var("JUDGE_CONTAINER_PORT")
        .unwrap_or_else(|_| DEFAULT_JUDGE_CONTAINER_PORT.to_string());
    let docker_address =
        dotenv::var("DOCKER_ADDRESS").unwrap_or_else(|_| DEFAULT_DOCKER_ADDRESS.to_string());
    Config {
        database_url,
        database_pool_size,
        judge_container_port,
        docker_address,
    }
}

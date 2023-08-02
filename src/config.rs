use once_cell::sync::Lazy;

pub static ENV_CONFIG: Lazy<Config> = Lazy::new(load_config);
pub struct Config {
    pub database_url: String,
    pub judge_container_port: String,
    pub docker_address: String,
    pub docker_image_name: String,
    pub docker_database_url: String,
    pub job_threads: usize,
    pub google_application_credentials: String,
}

pub fn load_config() -> Config {
    const DEFAULT_JUDGE_CONTAINER_PORT: &str = "8000";
    const DEFAULT_DOCKER_ADDRESS: &str = "http://localhost:2375";
    const DEFAULT_DOCKER_IMAGE_NAME: &str = "mofe-sandbox:2307";
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let judge_container_port = dotenv::var("JUDGE_CONTAINER_PORT")
        .unwrap_or_else(|_| DEFAULT_JUDGE_CONTAINER_PORT.to_string());
    let docker_address =
        dotenv::var("DOCKER_ADDRESS").unwrap_or_else(|_| DEFAULT_DOCKER_ADDRESS.to_string());
    let docker_image_name =
        dotenv::var("DOCKER_IMAGE_NAME").unwrap_or_else(|_| DEFAULT_DOCKER_IMAGE_NAME.to_string());
    let docker_database_url = dotenv::var("DOCKER_DATABASE_URL").unwrap_or(database_url.clone());
    let job_threads = dotenv::var("JOB_THREADS")
        .unwrap_or_else(|_| "3".to_string())
        .parse::<usize>()
        .unwrap();
    let google_application_credentials =
        dotenv::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or(String::new());
    Config {
        database_url,
        judge_container_port,
        docker_address,
        docker_image_name,
        docker_database_url,
        job_threads,
        google_application_credentials,
    }
}

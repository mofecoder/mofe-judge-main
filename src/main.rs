mod config;
mod db;
mod entities;
mod lang_cmd;
mod models;
mod repository;
mod task;
mod utils;
use anyhow::Result;
use bollard::API_DEFAULT_VERSION;
use config::ENV_CONFIG;
use futures::future::join_all;
use std::sync::Arc;


// TODO(magurotuna): スレッド数指定を柔軟に行うため、Tokio の RuntimeBuilder を使うよう書き換える
#[tokio::main()]
async fn main() -> Result<()> {
    let db_conn = Arc::new(db::new_pool(&ENV_CONFIG.database_url).await?);
    /*
    let docker_conn = Arc::new(bollard::Docker::connect_with_http(
        &ENV_CONFIG.docker_address,
        4,
        API_DEFAULT_VERSION,
    )?);
    */
    let docker_conn = Arc::new(bollard::Docker::connect_with_local(
        "/var/run/docker.sock",
        10,
        API_DEFAULT_VERSION,
    )?);
    let http_client = reqwest::Client::builder().build()?;

    let mut handles = Vec::new();
    for _ in 0..ENV_CONFIG.job_threads {
        let handle = tokio::spawn(task::gen_job(
            db_conn.clone(),
            docker_conn.clone(),
            http_client.clone(),
        ));
        handles.push(handle);
    }

    join_all(handles).await;

    Ok(())
}

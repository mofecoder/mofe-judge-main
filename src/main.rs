mod config;
mod db;
mod entities;
mod lang_cmd;
mod models;
mod repository;
mod task;
mod utils;
use anyhow::Result;
use core::time::Duration;
use futures::future::join_all;
use std::sync::Arc;

// TODO(magurotuna): ここの値も要検討
const JOB_THREADS: usize = 3;
const HTTP_TIMEOUT: u64 = 180;

// TODO(magurotuna): スレッド数指定を柔軟に行うため、Tokio の RuntimeBuilder を使うよう書き換える
#[tokio::main(worker_threads = 4)]
async fn main() -> Result<()> {
    let config = config::load_config()?;
    let db_conn = Arc::new(db::new_pool(&config).await?);
    let docker_conn = Arc::new(bollard::Docker::connect_with_http_defaults()?);
    let http_client = reqwest::Client::builder()
        .timeout(Duration::new(HTTP_TIMEOUT, 0))
        .build()?;

    let mut handles = Vec::new();
    for _ in 0..JOB_THREADS {
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

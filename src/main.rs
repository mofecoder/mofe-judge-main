mod config;
mod db;
mod models;
mod repository;
mod scoring;
mod task;
mod utils;

use anyhow::Result;
use futures::future::join_all;
use std::sync::Arc;

// TODO(magurotuna): ここの値も要検討
const JOB_THREADS: usize = 3;

// TODO(magurotuna): スレッド数指定を柔軟に行うため、Tokio の RuntimeBuilder を使うよう書き換える
#[tokio::main(worker_threads = 4)]
async fn main() -> Result<()> {
    let config = config::load_config()?;
    let db_conn = Arc::new(db::new_pool(&config).await?);
    let docker_conn = Arc::new(bollard::Docker::connect_with_unix_defaults()?);

    let mut handles = Vec::new();
    for _ in 0..JOB_THREADS {
        let db = Arc::clone(&db_conn);
        let docker = Arc::clone(&docker_conn);
        let handle = tokio::spawn(task::gen_job(db, docker));
        handles.push(handle);
    }

    join_all(handles).await;

    Ok(())
}

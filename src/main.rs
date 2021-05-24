mod config;
mod initializer;
mod lang_cmd;
mod schema;
mod utils;
use crate::config::*;
use crate::initializer::App;
use anyhow::Result;
use bollard::API_DEFAULT_VERSION;
use core::time::Duration;
use futures::future::join_all;
//use futures::future::join_all;
use std::sync::Arc;

#[macro_use]
mod wrapper;
pub use wrapper::error::*;
pub use wrapper::*;
#[macro_use]
extern crate diesel;

#[macro_use]
mod domain;
mod infra;
mod repositories;
// TODO(magurotuna): ここの値も要検討
const JOB_THREADS: usize = 3;
const HTTP_TIMEOUT: u64 = 180;
// TODO(magurotuna): スレッド数指定を柔軟に行うため、Tokio の RuntimeBuilder を使うよう書き換える
#[tokio::main(worker_threads = 4)]
async fn main() -> Result<()> {
    let mut handles = Vec::new();

    for _ in 0..JOB_THREADS {
        handles.push(tokio::spawn(async move {
            let conf = load_config();
            let http_client = reqwest::Client::builder()
                .timeout(Duration::new(HTTP_TIMEOUT, 0))
                .build()
                .unwrap();
            let app = App::new(conf);
            let docker_conn = Arc::new(
                bollard::Docker::connect_with_unix("/var/run/docker.sock", 10, API_DEFAULT_VERSION)
                    .unwrap(),
            );
            let task = domain::service::JudgeTaskRunner::new(
                app.clone(),
                docker_conn.clone(),
                http_client,
            );

            task.gen_job().await
        }));
    }

    join_all(handles).await;

    Ok(())
}

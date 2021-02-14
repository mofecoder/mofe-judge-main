mod config;
mod conn;
mod db;
mod models;
mod repository;
mod task;
mod utils;

use anyhow::Result;
use futures::future::join_all;
use models::CmdResultJSON;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

type JsonMapMutex = Mutex<HashMap<String, CmdResultJSON>>;

// TODO(magurotuna): ここの値も要検討
const JOB_THREADS: usize = 3;

// TODO(magurotuna): スレッド数指定を柔軟に行うため、Tokio の RuntimeBuilder を使うよう書き換える
#[tokio::main(core_threads = 4)]
async fn main() -> Result<()> {
    let config = config::load_config()?;
    let db_conn = Arc::new(db::new_pool(&config).await?);
    let docker_conn = Arc::new(bollard::Docker::connect_with_unix_defaults()?);
    let json_map_mutex = Arc::new(Mutex::new(HashMap::new()));
    tokio::spawn(conn::server(json_map_mutex.clone()));

    let mut handles = Vec::new();
    for _ in 0..JOB_THREADS {
        let db = Arc::clone(&db_conn);
        let docker = Arc::clone(&docker_conn);
        let json_map = Arc::clone(&json_map_mutex);
        let handle = tokio::spawn(task::gen_job(db, docker, json_map));
        handles.push(handle);
    }

    join_all(handles).await;

    Ok(())
}

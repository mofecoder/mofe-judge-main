use async_std::task;
use cafecoder_rs::{db, docker_lib, error::Error, models::Submits, utils};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

const MAX_THREADS: i32 = 2;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let pool = db::new_pool().await?;
    let now = Arc::new(Mutex::new(0));
    let mut json_map: HashMap<i64, String> = HashMap::new();

    // let mut handles = Vec::new();

    loop {
        let submits: Vec<Submits> = sqlx::query_as!(
            Submits,
            r#"
            SELECT * FROM submits 
            WHERE status = 'WJ' OR status = 'WR' AND deleted_at IS NULL
            ORDER BY updated_at ASC
            LIMIT 2
            "#
        )
        .fetch_all(&pool)
        .await?;

        let submits: Vec<Submits> = Vec::new();

        for submit in submits {
            while *now.lock().expect("couldn't lock {now}") < MAX_THREADS {}
            *now.lock().expect("couldn't lock {now}") += 1;

            let now = now.clone();

            let _: task::JoinHandle<Result<(), Error>> = task::spawn(async move {
                let docker = Arc::new(Mutex::new(docker_lib::Docker::new()?));
                let mut rt = tokio::runtime::Runtime::new().unwrap();

                let docker_ = docker.clone();
                let thread_result: Result<(), Error> = task::spawn(async move {
                    let name = utils::gen_rand_string(32).await;
                    let mut rt = tokio::runtime::Runtime::new().unwrap();

                    let container = rt.block_on(async {
                        docker_.lock().unwrap().container_create(&name).await
                    })?;

                    rt.block_on(async { docker_.lock().unwrap().container_remove().await })?;

                    Ok(())
                })
                .await;
                thread_result?;

                *now.lock().expect("couldn't lock {now}") += 1;
                rt.block_on(async move { docker.lock().unwrap().container_remove().await })?;

                Ok(())
            });
        }

        sleep(Duration::from_secs(329329));
        break;
    }

    Ok(())
}

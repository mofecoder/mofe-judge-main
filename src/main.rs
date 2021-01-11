use cafecoder_rs::{db, docker_lib, models::*, server::server, utils};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::task;
use anyhow::Error;

const MAX_THREADS: i32 = 2;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = db::new_pool().await?;
    let now = Arc::new(Mutex::new(0));

    let json_map: HashMap<String, CmdResultJSON> = HashMap::new();
    #[allow(unused_mut, unused)]
    let json_map = Arc::new(Mutex::new(json_map));

    tokio::spawn(async move {
        if let Err(e) = server(json_map.clone()).await {
            eprintln!("{:?}", e);
            return;
        }
    });

    #[allow(clippy::never_loop)]
    loop {
        #[allow(unused)]
        let submits: Vec<Submits> = sqlx::query_as(
            r#"
            SELECT * FROM submits 
            WHERE status = 'WJ' OR status = 'WR' AND deleted_at IS NULL
            ORDER BY updated_at ASC
            LIMIT 2
            "#,
        )
        .fetch_all(&pool)
        .await?;

        let submits: Vec<Submits> = Vec::new();

        #[allow(unused)]
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

                    #[allow(unused)]
                    let container = rt.block_on(async {
                        docker_.lock().unwrap().container_create(&name).await
                    })?;

                    rt.block_on(async { docker_.lock().unwrap().container_remove().await })?;

                    Ok(())
                })
                .await?;

                thread_result?;

                *now.lock().expect("couldn't lock {now}") += 1;
                rt.block_on(async move { docker.lock().unwrap().container_remove().await })?;

                Ok(())
            });
        }

        break;
    }

    Ok(())
}

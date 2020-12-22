use async_std::{future, task};
use cafecoder_rs::{db, docker_lib, models::Submits, utils};
use sqlx::{MySql, Pool};
use std::{thread::sleep, time::Duration};
use tokio::prelude::*;

const MAX_THREADS: i32 = 2;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut pool = db::new_pool().await?;
    let mut now = 0;
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

        for submit in submits {
            while now < MAX_THREADS {}
            now -= 1;

            task::spawn(async {
                let handle = task::spawn(async move {
                    let name = utils::gen_rand_string(32).await;
                    let mut rt = tokio::runtime::Runtime::new().unwrap();

                    let docker = docker_lib::Docker::new();
                    if let Err(e) = docker {
                        println!("{:?}", e);
                        return;
                    }
                    let mut docker = docker.unwrap();

                    let container = rt.block_on(async { docker.container_create(&name).await });
                    if let Err(e) = container {
                        println!("{:?}", e);
                        return;
                    }

                    if let Err(e) = rt.block_on(async { docker.container_remove().await }) {
                        println!("{:?}", e);
                        return;
                    }
                });

                
            });


        }

        sleep(Duration::from_secs(329329));
        break;
    }

    Ok(())
}

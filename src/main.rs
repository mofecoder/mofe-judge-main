mod config;
mod db;
mod docker_lib;
mod models;
mod repository;
mod utils;

use anyhow::Result;
use repository::SubmitRepository;
use std::time::Duration;
use tokio::time::delay_for;

#[tokio::main(core_threads = 2)]
async fn main() -> Result<()> {
    let config = config::load_config()?;
    let pool = db::new_pool(&config).await?;

    #[allow(clippy::never_loop)]
    loop {
        let submits = pool.get_submits().await?;

        // submits を2つずつに分割
        for chunk in submits.chunks(2) {
            let _first = chunk.get(0);
            let _second = chunk.get(1);

            // submit の情報 (first, second) を Docker::new に渡すべき？
            let container1 = docker_lib::Docker::new()?;
            let container2 = docker_lib::Docker::new()?;

            // container1, 2 の実行が終わるのを同時に待つ
            futures::try_join!(container1, container2)?;
        }

        delay_for(Duration::from_secs(329329)).await;
    }
}

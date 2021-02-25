// TODO: 実装が終わったらこのallowディレクティブを削除する
#![allow(unused)]

use crate::db::DbPool;
use crate::models::Submit;
use anyhow::{bail, Result};
use bollard::container::{Config, CreateContainerOptions, RemoveContainerOptions};
use bollard::models::HostConfig;
use bollard::service::ContainerCreateResponse;
use bollard::Docker;
use futures::future::FutureExt;
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

// submit が取得できなかったときの次の取得までの間隔
const INTERVAL: Duration = Duration::from_secs(1);

pub async fn gen_job(db_conn: Arc<DbPool>, docker_conn: Arc<Docker>) {
    // この `task` が 1 実行単位
    let task = || {
        let db_conn = Arc::clone(&db_conn);
        let docker_conn = Arc::clone(&docker_conn);
        async move {
            let task = JudgeTask::new(db_conn, docker_conn);

            // 提出を取得
            let submit = match task.fetch_submit().await {
                Ok(s) => s,
                Err(_) => {
                    // TODO(magurotuna): 提出が取得できなかった場合は 1 秒待って次の実行に移る
                    sleep(INTERVAL).await;
                    bail!("Couldn't find an unjudged submit.");
                }
            };

            // コンテナ作成
            // TODO(magurotuna): コンテナ名をちゃんとする UUIDを発行？
            let container_name = "DUMMY_NAME";
            let container = task.create_container(container_name).await?;

            /////////////////////////////////////////////////////////////////
            //
            // ここでコンテナに対するリクエストなどの処理を行う
            //
            /////////////////////////////////////////////////////////////////

            // コンテナを削除
            task.remove_container(container_name).await?;
            // ジャッジ終了としてDBを更新
            task.save_as_finished(submit.id).await?;

            Ok(())
        }
    };

    // stream::unfold をすることで、1 実行単位である `task` を延々と繰り返すような Stream を作る
    let mut stream = stream::unfold((), move |_| {
        // カッコが続いて見づらくなるので Unit に置き換えて多少見やすくなるようにしている
        type Unit = ();
        const UNIT: () = ();
        fn mapper(task_result: Result<Unit>) -> Option<(Result<Unit>, Unit)> {
            Some((task_result, UNIT))
        }
        task().map(mapper)
    })
    .boxed();

    while let Some(_task_result) = stream.next().await {
        // 1回1回の task の実行結果を使って何かやりたければここに書く
        // ログ出力とか？
    }
}

/// 1つの submit に対するジャッジの処理を担当する
struct JudgeTask {
    db_conn: Arc<DbPool>,
    docker_conn: Arc<Docker>,
}

impl JudgeTask {
    fn new(db_conn: Arc<DbPool>, docker_conn: Arc<Docker>) -> Self {
        Self {
            db_conn,
            docker_conn,
        }
    }

    /// 未ジャッジの提出のうち、もっとも古いもの1件を取得する。
    /// その1件のステータスを「ジャッジ中」にする
    async fn fetch_submit(&self) -> Result<Submit> {
        todo!()
    }

    /// Docker コンテナを指定された名前で立ち上げる
    async fn create_container(&self, name: &str) -> Result<(ContainerCreateResponse, String)> {
        const IMAGE: &str = "cafecoder";
        let options = Some(CreateContainerOptions { name });
        let config = Config {
            image: Some(IMAGE),
            host_config: Some(HostConfig {
                memory: Some(2_147_483_648_i64),
                pids_limit: Some(512_i64),
                ..Default::default()
            }),
            ..Default::default()
        };

        let inspect = self.docker_conn.inspect_container(name, None).await?;

        let network_settings = inspect
            .network_settings
            .expect("couldn't get network_settings");
        let ip_addr = network_settings
            .ip_address
            .expect("couldn't get IP address");

        let res = self.docker_conn.create_container(options, config).await?;
        Ok((res, ip_addr))
    }

    /// ジャッジの進捗をDBに保存する
    /// TODO: 引数に追加情報が必要だと思うので、必要に応じて追加する
    async fn save_progress(&self, submit_id: i64) -> Result<()> {
        todo!()
    }

    /// Docker コンテナを削除する
    async fn remove_container(&self, name: &str) -> Result<()> {
        let options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        self.docker_conn
            .remove_container(name, Some(options))
            .await?;
        Ok(())
    }

    /// ジャッジが正常に終了した旨を `submit_id` のレコードに保存する
    async fn save_as_finished(&self, submit_id: i64) -> Result<()> {
        todo!()
    }
}

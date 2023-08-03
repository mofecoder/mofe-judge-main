use crate::{
    config::ENV_CONFIG,
    db::DbPool,
    entities,
    lang_cmd::LANG_CMD,
    models::container::{
        CompileRequest, CompileResponse, DownloadRequest, JudgeRequest, JudgeResponse, Problem,
        Testcase,
    },
    utils,
};

use crate::repository::CafeCoderDb;
use anyhow::{bail, Result};
use bollard::models::{Mount, MountTypeEnum};
use bollard::{
    container::{Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions},
    models::HostConfig,
    service::ContainerCreateResponse,
    Docker,
};
use futures::{
    future::FutureExt,
    stream::{self, StreamExt},
};
use reqwest::{header, Client, Response, StatusCode};
use std::clone::Clone;
use std::default::Default;
use std::option::Option::{None, Some};
use std::result::Result::{Err, Ok};
use std::string::{String, ToString};
use std::vec::Vec;
use std::{env, sync::Arc, time::Duration};
use tokio::time::sleep;

// submit が取得できなかったときの次の取得までの間隔
const INTERVAL: Duration = Duration::from_secs(1);
const MEM_LIMIT: i32 = 1_024_000; // 1,024,000KB(1,024KB)

pub async fn gen_job(db_conn: Arc<DbPool>, docker_conn: Arc<Docker>, http_client: Client) {
    // この `task` が 1 実行単位
    let task = || {
        let db_conn = Arc::clone(&db_conn);
        let docker_conn = Arc::clone(&docker_conn);
        let http_client = http_client.clone();
        async move {
            let task = JudgeTask::new(db_conn, docker_conn, http_client);
            // 提出を取得
            let submit = match task.fetch_submit().await {
                Ok(s) => s,
                Err(_) => {
                    // TODO(magurotuna): 提出が取得できなかった場合は 1 秒待って次の実行に移る
                    sleep(INTERVAL).await;
                    bail!("Couldn't find an unjudged submit.");
                }
            };
            // TODO(magurotuna): コンテナ名をちゃんとする UUIDを発行？
            let container_name = format!("DUMMY_NAME_{}", utils::gen_rand_string(6));

            match execute_task(&task, &submit, &container_name).await {
                Ok(()) => (),
                Err(e) => {
                    eprint!("{}", e);
                    sleep(INTERVAL).await;
                    task.save_internal_error(submit.id).await?;
                    //task.remove_container(&container_name).await?;
                    bail!("internal error");
                }
            }
            Ok(())
        }
    };

    // stream::unfold をすることで、1 実行単位である `task` を延々と繰り返すような Stream を作る
    let mut stream = stream::unfold((), move |_| {
        // カッコが続いて見づらくなるので Unit に置き換えて多少見やすくなるようにしている

        const UNIT: () = ();
        task().map(|task_result| Some((task_result, UNIT)))
    })
    .boxed();

    while let Some(_task_result) = stream.next().await {
        // 1回1回の task の実行結果を使って何かやりたければここに書く
        // ログ出力とか？
    }
}
async fn execute_task(
    task: &JudgeTask,
    submit: &entities::Submission,
    container_name: &str,
) -> Result<()> {
    let command = match LANG_CMD.get(&submit.lang) {
        Some(command) => command,
        None => {
            // statusをIEへ
            task.save_internal_error(submit.id).await?;

            bail!("Couldn't find a language command setting.");
        }
    };

    // リジャッジなら過去の testcase_results をすべて消す。
    if submit.status == "WR" {
        task.delete_testcase_results(submit.id).await?;
    }

    // コンテナ作成

    let (_container, ip_addr) = task.create_container(&container_name).await?;
    // テストケース、問題を取得

    let problem = task.fetch_problem(submit.problem_id).await?;
    let testcases = task.fetch_testcases(submit.problem_id).await?;

    let req = generate_judge_request(submit.id, &command.run, problem, testcases);

    let download_response = task
        .request_download(
            &ip_addr,
            &DownloadRequest {
                submit_id: submit.id,
                code_path: submit.path.clone(),
                filename: command.file_name.clone(),
            },
        )
        .await?;

    if download_response.status() != StatusCode::OK {
        task.remove_container(&container_name).await?;
        return Err(anyhow::anyhow!("Download failed"));
    }

    let compile_response = task
        .request_compile(
            &ip_addr,
            &CompileRequest {
                submit_id: submit.id,
                cmd: command.compile.clone(),
            },
        )
        .await?;
    // コンパイルエラーはコンテナの中で処理をしているはずなので ok
    if !compile_response.0.ok {
        task.remove_container(&container_name).await?;
        return Ok(());
    }

    task.save_compiling(submit.id).await?;
    let _judge_response = task.request_judge(&ip_addr, &req).await?;
    // TODO judgeレスポンスによる処理
    // コンテナを削除
    task.remove_container(&container_name).await?;

    Ok(())
}
fn generate_judge_request(
    submit_id: i64,
    cmd: &str,
    problem: entities::Problem,
    testcases: Vec<entities::Testcase>,
) -> JudgeRequest {
    let request_testcases = testcases
        .iter()
        .map(|t| Testcase {
            testcase_id: t.id,
            name: t.name.clone().unwrap_or_default(),
        })
        .collect();
    let request_problem = Problem {
        problem_id: problem.id,
        uuid: problem.uuid.unwrap_or_default(),
        checker_path: problem
            .checker_path
            .unwrap_or_else(|| "checker_path/wcmp.cpp".to_string()),
    };
    JudgeRequest {
        submit_id,
        cmd: cmd.to_string(),
        time_limit: problem.execution_time_limit,
        mem_limit: MEM_LIMIT,
        testcases: request_testcases,
        problem: request_problem,
    }
}

/// 1つの submit に対するジャッジの処理を担当する
#[derive(Debug)]
struct JudgeTask {
    db_conn: Arc<DbPool>,
    docker_conn: Arc<Docker>,
    http_client: Client,
}

impl JudgeTask {
    fn new(db_conn: Arc<DbPool>, docker_conn: Arc<Docker>, http_client: Client) -> Self {
        Self {
            db_conn,
            docker_conn,
            http_client,
        }
    }

    /// 未ジャッジの提出のうち、もっとも古いもの1件を取得する。
    /// その1件のステータスを「ジャッジ中」にする
    async fn fetch_submit(&self) -> Result<entities::Submission> {
        let mut conn = self.db_conn.begin().await?;
        let submission = CafeCoderDb::get_submission(&mut conn).await?;
        CafeCoderDb::update_status(&mut conn, submission.id, "CP").await?;
        conn.commit().await?;
        Ok(submission)
    }
    async fn fetch_problem(&self, problem_id: i64) -> Result<entities::Problem> {
        let mut conn = self.db_conn.acquire().await?;
        Ok(CafeCoderDb::fetch_problem(&mut conn, problem_id).await?)
    }

    async fn fetch_testcases(&self, problem_id: i64) -> Result<Vec<entities::Testcase>> {
        let mut conn = self.db_conn.acquire().await?;
        Ok(CafeCoderDb::fetch_testcases(&mut conn, problem_id).await?)
    }

    async fn delete_testcase_results(&self, submit_id: i64) -> Result<()> {
        let mut conn = self.db_conn.acquire().await?;
        Ok(CafeCoderDb::delete_testcase_results(&mut conn, submit_id).await?)
    }

    async fn save_compiling(&self, submission_id: i64) -> Result<()> {
        let mut conn = self.db_conn.acquire().await?;
        CafeCoderDb::update_status(&mut conn, submission_id, "CP").await?;
        Ok(())
    }

    /// Docker コンテナを指定された名前で立ち上げる
    async fn create_container(&self, name: &str) -> Result<(ContainerCreateResponse, String)> {
        let options = Some(CreateContainerOptions {
            name,
            platform: None,
        });
        const SERVICE_ACCOUNT_PATH: &str = "/service-account.json";
        let config = Config {
            image: Some(ENV_CONFIG.docker_image_name.clone()),
            host_config: Some(HostConfig {
                memory: Some(2_147_483_648_i64),
                pids_limit: Some(512_i64),
                privileged: Some(true),
                mounts: Some(vec![Mount {
                    typ: Some(MountTypeEnum::BIND),
                    source: Some(String::from(
                        env::current_dir()
                            .unwrap()
                            .join(&ENV_CONFIG.google_application_credentials)
                            .to_str()
                            .unwrap(),
                    )),
                    target: Some(String::from(SERVICE_ACCOUNT_PATH)),
                    read_only: Some(true),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            env: Some(vec![
                format!("DATABASE_URL={}", ENV_CONFIG.database_url),
                format!("GOOGLE_APPLICATION_CREDENTIALS={}", SERVICE_ACCOUNT_PATH),
            ]),
            ..Default::default()
        };
        let res = self.docker_conn.create_container(options, config).await?;

        self.docker_conn
            .start_container(name, None::<StartContainerOptions<String>>)
            .await?;
        let inspect = self.docker_conn.inspect_container(name, None).await?;

        let network_settings = inspect
            .network_settings
            .expect("couldn't get network_settings");
        let ip_addr = network_settings
            .ip_address
            .expect("couldn't get IP address");

        // TODO: コンテナが立ち上がったかどうかのチェック
        tokio::time::sleep(Duration::new(1, 0)).await;

        Ok((res, ip_addr))
    }

    pub async fn request_compile(
        &self,
        ip_addr: &str,
        req: &CompileRequest,
    ) -> Result<CompileResponse, anyhow::Error> {
        dbg!(serde_json::to_string(req).unwrap());
        let resp = self
            .http_client
            .post(&format!(
                "http://{}:{}/compile",
                &ip_addr, &ENV_CONFIG.judge_container_port
            ))
            .json(&req)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if resp.status() != StatusCode::OK {
            anyhow::bail!(format!(
                "{}\n{}",
                "compile: response status code was not 200 OK",
                resp.text().await?
            ));
        }

        let resp = resp.json().await?;

        Ok(resp)
    }

    pub async fn request_download(
        &self,
        ip_addr: &str,
        req: &DownloadRequest,
    ) -> Result<Response, anyhow::Error> {
        dbg!(serde_json::to_string(req).unwrap());
        let resp = self
            .http_client
            .post(&format!(
                "http://{}:{}/download",
                &ip_addr, &ENV_CONFIG.judge_container_port
            ))
            .json(&req)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        Ok(resp)
    }
    pub async fn request_judge(
        &self,
        ip_addr: &str,
        req: &JudgeRequest,
    ) -> Result<JudgeResponse, anyhow::Error> {
        dbg!(serde_json::to_string(req).unwrap());
        let resp = self
            .http_client
            .post(&format!(
                "http://{}:{}/judge",
                &ip_addr, &ENV_CONFIG.judge_container_port
            ))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&req)
            .send()
            .await?;

        if resp.status() != StatusCode::OK {
            bail!(format!(
                "judge: response status code was not 200 OK\n{}\n",
                resp.text().await?
            ));
        }

        Ok(resp.json().await?)
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

    async fn save_internal_error(&self, submit_id: i64) -> Result<()> {
        let mut conn = self.db_conn.begin().await?;
        let row = CafeCoderDb::update_status(&mut conn, submit_id, "IE").await?;
        if row == 1 {
            conn.commit().await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Update failed"))
        }
    }
}

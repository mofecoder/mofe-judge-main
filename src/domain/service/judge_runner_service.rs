use crate::domain::model::*;
use crate::domain::service::*;
use crate::initializer::*;
use crate::lang_cmd::LANG_CMD;
use crate::repositories::*;
use crate::utils::{self};
use crate::wrapper::*;
use bollard::Docker;
use chrono::*;
use futures::stream;
use futures::*;
use reqwest::{Client, StatusCode};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct JudgeTaskRunner {
    problem_repo: ProblemsRepository,
    submit_repo: SubmitRepository,
    testcase_repo: TestcasesRepository,
    testcase_result_repo: TestcasesResultRepository,
    docker_conn: Arc<Docker>,
    http_client: Client,
}

const INTERVAL: Duration = Duration::from_secs(1);
const MEM_LIMIT: i32 = 1_024_000; // 1,024,000KB(1,024KB)
impl JudgeTaskRunner {
    pub fn new(app: App, docker_conn: Arc<Docker>, http_client: Client) -> Self {
        Self {
            problem_repo: app.repositories.problem_repo.as_ref().clone(),
            submit_repo: app.repositories.submit_repo.as_ref().clone(),
            testcase_repo: app.repositories.testcase_repo.as_ref().clone(),
            testcase_result_repo: app.repositories.testcase_result_repo.as_ref().clone(),
            docker_conn,
            http_client,
        }
    }

    pub async fn gen_job(&self) -> Result<(), GeneralError> {
        // この `task` が 1 実行単位
        let task = || {
            async move {
                let task: JudgeTask = JudgeTask::new(
                    self.problem_repo.clone(),
                    self.submit_repo.clone(),
                    self.testcase_repo.clone(),
                    self.testcase_result_repo.clone(),
                    self.docker_conn.clone(),
                    self.http_client.clone(),
                );
                // 提出を取得
                let submit = match task.fetch_submit().await {
                    Ok(s) => s,
                    Err(_) => {
                        // TODO(magurotuna): 提出が取得できなかった場合は 1 秒待って次の実行に移る
                        sleep(INTERVAL).await;
                        return GeneralError::only("Couldn't find an unjudged submit.");
                    }
                };
                // TODO(magurotuna): コンテナ名をちゃんとする UUIDを発行？
                let container_name = format!("DUMMY_NAME_{}", utils::gen_rand_string(6));

                match self.execute_task(&task, &submit, &container_name).await {
                    Ok(()) => (),
                    Err(e) => {
                        eprint!("{}", e.error_type());
                        sleep(INTERVAL).await;
                        match task.save_internal_error(submit.id).await {
                            Ok(_) => (()),
                            _ => {
                                return GeneralError::only("internal error");
                            }
                        };
                        //task.remove_container(&container_name).await?;
                        return GeneralError::only("internal error");
                    }
                }
                GeneralError::only("ok")
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
        Ok(())
    }
    async fn execute_task(
        &self,
        task: &JudgeTask,
        submit: &Submit,
        container_name: &str,
    ) -> Result<(), GeneralError> {
        let command = match LANG_CMD.get(&submit.lang) {
            Some(command) => command,
            None => {
                // statusをIEへ
                task.save_internal_error(submit.id.clone()).await?;

                return Err(GeneralError::only(
                    "Couldn't find a language command setting.",
                ));
            }
        };

        // リジャッジなら過去の testcase_results をすべて消す。
        match submit.status {
            Status::WR => {
                task.delete_testcase_results(&submit.id).await?;
            }
            _ => (),
        }

        // コンテナ作成

        let (_container, ip_addr) = match task.create_container(&container_name).await {
            Ok((c, i)) => (c, i),
            Err(e) => return Err(GeneralError::new(DockerError::InternalError, e)),
        };
        // テストケース、問題を取得

        let problem = task.fetch_problem(&submit.problem_id).await?;
        let testcases = task.fetch_testcases(&submit.problem_id).await?;

        let req = self.generate_judge_request(submit.id.clone(), &command.run, problem, testcases);

        let download_response = task
            .request_download(
                &ip_addr,
                &DownloadRequest {
                    submit_id: submit.id.clone(),
                    code_path: submit.path.clone(),
                    filename: command.file_name.clone(),
                },
            )
            .await?;

        if download_response.status() != StatusCode::OK {
            return Err(GeneralError::only("Download failed"));
        }

        let compile_response = task
            .request_compile(
                &ip_addr,
                &CompileRequest {
                    submit_id: submit.id.clone(),
                    cmd: command.compile.clone(),
                },
            )
            .await?;
        // コンパイルエラーはコンテナの中で処理をしているはずなので ok
        if !compile_response.0.ok {
            return Ok(());
        }

        let _judge_response = task.request_judge(&ip_addr, &req).await?;
        // TODO judgeレスポンスによる処理
        // コンテナを削除
        task.remove_container(&container_name).await?;

        Ok(())
    }
    fn generate_judge_request(
        &self,
        submit_id: SubmitId,
        cmd: &str,
        problem: Problem,
        testcases: Vec<Testcase>,
    ) -> JudgeRequest {
        let request_testcases = testcases
            .iter()
            .map(|t| Testcase {
                id: t.id.clone(),
                input: None,
                output: None,
                explanation: None,
                problem_id: ProblemId(-1),
                name: Some(t.name.clone().unwrap_or_default()),
                created_at: Local::now().naive_local(),
                updated_at: Local::now().naive_local(),
                deleted_at: None,
            })
            .collect();
        let request_problem = Problem {
            id: problem.id,
            uuid: Some(problem.uuid.unwrap_or_default()),
            checker_path: Some(
                problem
                    .checker_path
                    .unwrap_or_else(|| "checker_path/wcmp.cpp".to_string()),
            ),
            ..problem
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
}

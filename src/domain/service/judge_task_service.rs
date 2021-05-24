use crate::config::ENV_CONFIG;
use crate::domain::model::*;
use crate::repositories::*;
use crate::wrapper::*;
use anyhow::Result;
use bollard::{
    container::{Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions},
    models::HostConfig,
    service::ContainerCreateResponse,
    Docker,
};
use reqwest::{header, Client, Response, StatusCode};
use std::sync::Arc;
use std::time::Duration;
pub struct JudgeTask {
    pub problem_repo: ProblemsRepository,
    pub submit_repo: SubmitRepository,
    pub testcase_repo: TestcasesRepository,
    pub testcase_result_repo: TestcasesResultRepository,
    pub docker_conn: Arc<Docker>,
    pub http_client: Client,
}

/// 1つの submit に対するジャッジの処理を担当する
impl JudgeTask {
    pub fn new(
        problem_repo: ProblemsRepository,
        submit_repo: SubmitRepository,
        testcase_repo: TestcasesRepository,
        testcase_result_repo: TestcasesResultRepository,
        docker_conn: Arc<Docker>,
        http_client: Client,
    ) -> Self {
        Self {
            problem_repo,
            submit_repo,
            testcase_repo,
            testcase_result_repo,
            docker_conn,
            http_client,
        }
    }

    /// 未ジャッジの提出のうち、もっとも古いもの1件を取得する。
    /// その1件のステータスを「ジャッジ中」にする
    pub async fn fetch_submit(&self) -> Result<Submit, GeneralError> {
        let mut submit = self.submit_repo.pop_queing_submit().await?;
        submit.status = Status::WIP;
        self.submit_repo.save(submit.clone()).await?;
        Ok(submit.clone())
    }
    pub async fn fetch_problem(&self, problem_id: &ProblemId) -> Result<Problem, GeneralError> {
        Ok(self.problem_repo.find_by_id(problem_id).await?)
    }

    pub async fn fetch_testcases(
        &self,
        problem_id: &ProblemId,
    ) -> Result<Vec<Testcase>, GeneralError> {
        Ok(self.testcase_repo.find_by_problem_id(problem_id).await?)
    }

    pub async fn delete_testcase_results(&self, submit_id: &SubmitId) -> Result<(), GeneralError> {
        Ok(self.testcase_result_repo.logical_delete(submit_id).await?)
    }

    /// Docker コンテナを指定された名前で立ち上げる
    pub async fn create_container(&self, name: &str) -> Result<(ContainerCreateResponse, String)> {
        const IMAGE: &str = "cafecoder_docker:2104";

        let options = Some(CreateContainerOptions { name });
        let config = Config {
            image: Some(IMAGE),
            host_config: Some(HostConfig {
                memory: Some(2_147_483_648_i64),
                pids_limit: Some(512_i64),
                privileged: Some(true),
                ..Default::default()
            }),
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
    ) -> Result<CompileResponse, GeneralError> {
        dbg!(serde_json::to_string(req).unwrap());
        let resp = match self
            .http_client
            .post(&format!(
                "http://{}:{}/compile",
                &ip_addr, &ENV_CONFIG.judge_container_port
            ))
            .json(&req)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
        {
            Ok(r) => r,
            _ => {
                return Err(GeneralError::only("request_compile_error"));
            }
        };

        if resp.status() != StatusCode::OK {
            GeneralError::only("compile: response status code was not 200 OK");
        }

        match resp.json().await {
            Ok(r) => Ok(r),
            _ => {
                return Err(GeneralError::only("request_compile_serialize_error"));
            }
        }
    }

    pub async fn request_download(
        &self,
        ip_addr: &str,
        req: &DownloadRequest,
    ) -> Result<Response, GeneralError> {
        dbg!(serde_json::to_string(req).unwrap());
        let resp = match self
            .http_client
            .post(&format!(
                "http://{}:{}/download",
                &ip_addr, &ENV_CONFIG.judge_container_port
            ))
            .json(&req)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
        {
            Ok(r) => r,
            _ => {
                return Err(GeneralError::only("request_download_error"));
            }
        };

        Ok(resp)
    }
    pub async fn request_judge(
        &self,
        ip_addr: &str,
        req: &JudgeRequest,
    ) -> Result<JudgeResponse, GeneralError> {
        dbg!(serde_json::to_string(req).unwrap());
        let resp = match self
            .http_client
            .post(&format!(
                "http://{}:{}/judge",
                &ip_addr, &ENV_CONFIG.judge_container_port
            ))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&req)
            .send()
            .await
        {
            Ok(r) => r,
            _ => {
                return Err(GeneralError::only("request_judge_error"));
            }
        };

        if resp.status() != StatusCode::OK {
            GeneralError::only("judge: response status code was not 200 OK\n");
        }

        match resp.json().await {
            Ok(r) => Ok(r),
            _ => {
                return Err(GeneralError::only("request_judge_serialize_error"));
            }
        }
    }

    /// Docker コンテナを削除する
    pub async fn remove_container(&self, name: &str) -> Result<(), GeneralError> {
        let options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };
        match self.docker_conn.remove_container(name, Some(options)).await {
            Ok(r) => r,
            _ => {
                return Err(GeneralError::only("remove_container_error"));
            }
        };
        Ok(())
    }

    pub async fn save_internal_error(&self, submit_id: SubmitId) -> Result<(), GeneralError> {
        let mut submit = self.submit_repo.find_by_id(&submit_id).await?;
        submit.status = Status::IE;
        match self.submit_repo.save(submit).await {
            Ok(_) => Ok(()),
            _ => {
                return Err(GeneralError::only("save_internal_error"));
            }
        }
    }
}

use crate::config::Config;
use crate::infra::*;
use crate::repositories::{self, *};
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {
    pub problem_repo: Arc<repositories::ProblemsRepository>,
    pub submit_repo: Arc<repositories::SubmitRepository>,
    pub testcase_repo: Arc<repositories::TestcasesRepository>,
    pub testcase_result_repo: Arc<repositories::TestcasesResultRepository>,
}

#[derive(Clone)]
pub struct App {
    pub config: Config,
    pub repositories: Repositories,
}

impl App {
    pub fn new(conf: Config) -> Self {
        let pool = DBExecutor::new(conf.clone().database_url, conf.clone().database_pool_size);
        App {
            config: conf.clone(),
            repositories: Repositories::new(DBConnector::new(pool)),
        }
    }
}

impl Repositories {
    pub fn new(dbconn: DBConnector) -> Self {
        Self {
            problem_repo: Arc::new(ProblemsRepository::new(dbconn.clone())),
            submit_repo: Arc::new(SubmitRepository::new(dbconn.clone())),
            testcase_repo: Arc::new(TestcasesRepository::new(dbconn.clone())),
            testcase_result_repo: Arc::new(TestcasesResultRepository::new(dbconn.clone())),
        }
    }
}

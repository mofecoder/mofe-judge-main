use crate::domain::model::*;
use crate::infra::*;
use crate::schema::*;
use crate::wrapper::*;
use async_trait::async_trait;
use chrono::Local;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::update;

#[derive(Clone)]
pub struct TestcasesResultRepository {
    db: DBConnector,
}

#[derive(Clone, Queryable)]
pub struct TestcaseResultRecord {
    id: i64,
    submit_id: i64,
    testcase_id: i64,
    status: String,
    execution_time: i32,
    execution_memory: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

/*
impl TestcaseResultRecord {
    fn from_model(result: TestcaseResult) -> TestcaseResultRecord {
        TestcaseResultRecord {
            id: testcase.id.0,
            problem_id: testcase.problem_id.0,
            name: testcase.name,
            input: testcase.input,
            output: testcase.output,
            explanation: testcase.explanation,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
            deleted_at: testcase.deleted_at,
        }
    }
    fn to_model(&self) -> TestcaseResult {
        Testcase {
            id: TestcaseId(self.id),
            problem_id: ProblemId(self.problem_id),
            name: self.name,
            input: self.input,
            output: self.output,
            explanation: self.explanation,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}
*/
#[async_trait]
pub trait ITestcaseResultRepository {
    fn new(db: DBConnector) -> Self;
    async fn logical_delete(&self, submit_id: &SubmitId) -> Result<(), GeneralError>;
}

#[async_trait]
impl ITestcaseResultRepository for TestcasesResultRepository {
    fn new(db: DBConnector) -> Self {
        Self { db }
    }
    async fn logical_delete(&self, submit_id: &SubmitId) -> Result<(), GeneralError> {
        self.db
            .execute(
                update(
                    testcase_results::table
                        .filter(testcase_results::submit_id.eq(submit_id.0))
                        .filter(testcase_results::deleted_at.is_not_null()),
                )
                .set(testcase_results::deleted_at.eq(Local::now().naive_local())),
            )
            .await?;
        Ok(())
    }
}

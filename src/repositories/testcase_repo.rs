use crate::domain::model::*;
use crate::infra::*;
use crate::schema::*;
use crate::wrapper::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone)]
pub struct TestcasesRepository {
    db: DBConnector,
}

#[derive(Clone, Queryable)]
pub struct TestcaseRecord {
    id: i64,
    problem_id: i64,
    name: Option<String>,
    input: Option<String>,
    output: Option<String>,
    explanation: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

impl TestcaseRecord {
    #[allow(dead_code)]
    fn from_model(testcase: Testcase) -> TestcaseRecord {
        TestcaseRecord {
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

    fn to_model(&self) -> Testcase {
        Testcase {
            id: TestcaseId(self.id),
            problem_id: ProblemId(self.problem_id),
            name: self.name.clone(),
            input: self.input.clone(),
            output: self.output.clone(),
            explanation: self.explanation.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[async_trait]
pub trait ITestcaseRepository {
    fn new(db: DBConnector) -> Self;
    async fn find_by_problem_id(
        &self,
        problem_id: &ProblemId,
    ) -> Result<Vec<Testcase>, GeneralError>;
}

#[async_trait]
impl ITestcaseRepository for TestcasesRepository {
    fn new(db: DBConnector) -> Self {
        Self { db }
    }
    async fn find_by_problem_id(
        &self,
        problem_id: &ProblemId,
    ) -> Result<Vec<Testcase>, GeneralError> {
        let records = self
            .db
            .load::<TestcaseRecord, _>(
                testcases::table
                    .filter(testcases::problem_id.eq(problem_id.0))
                    .filter(testcases::deleted_at.is_not_null()),
            )
            .await?;
        Ok(records.into_iter().map(|r| r.to_model()).collect())
    }
}

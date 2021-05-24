use crate::domain::model::*;
use crate::infra::*;
use crate::schema::*;
use crate::wrapper::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::*;

#[derive(Clone)]
pub struct ProblemsRepository {
    db: DBConnector,
}

#[derive(Clone, Queryable)]
pub struct ProblemRecord {
    id: i64,
    slug: Option<String>,
    name: Option<String>,
    contest_id: Option<i64>,
    writer_user_id: i64,
    position: Option<String>,
    uuid: Option<String>,
    difficulty: String,
    execution_time_limit: i32,
    statement: String,
    constraints: String,
    input_format: String,
    output_format: String,
    checker_path: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

impl ProblemRecord {
    #[allow(dead_code)]
    fn from_model(problem: Problem) -> ProblemRecord {
        ProblemRecord {
            id: problem.id.0,
            slug: problem.slug,
            name: problem.name,
            contest_id: problem.contest_id.map(|c| c.0),
            writer_user_id: problem.writer_user_id.0,
            position: problem.position,
            uuid: problem.uuid.map(|u| u.to_hyphenated().to_string()),
            difficulty: problem.difficulty,
            execution_time_limit: problem.execution_time_limit,
            statement: problem.statement,
            constraints: problem.constraints,
            input_format: problem.input_format,
            output_format: problem.output_format,
            checker_path: problem.checker_path,
            created_at: problem.created_at,
            updated_at: problem.updated_at,
            deleted_at: problem.deleted_at,
        }
    }
    fn to_model(&self) -> Problem {
        Problem {
            id: ProblemId(self.id),
            slug: self.slug.clone(),
            name: self.name.clone(),
            contest_id: self.contest_id.map(|c| ContestId(c)),
            writer_user_id: UserId(self.writer_user_id),
            position: self.position.clone(),
            uuid: self
                .uuid
                .clone()
                .map(|u| Uuid::parse_str(&u).unwrap_or(Uuid::new_v4())),
            difficulty: self.difficulty.clone(),
            execution_time_limit: self.execution_time_limit,
            statement: self.statement.clone(),
            constraints: self.constraints.clone(),
            input_format: self.input_format.clone(),
            output_format: self.output_format.clone(),
            checker_path: self.checker_path.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[async_trait]
pub trait IProblemRepository {
    fn new(db: DBConnector) -> Self;
    async fn find_by_id(&self, problem_id: &ProblemId) -> Result<Problem, GeneralError>;
}

#[async_trait]
impl IProblemRepository for ProblemsRepository {
    fn new(db: DBConnector) -> Self {
        Self { db }
    }
    async fn find_by_id(&self, problem_id: &ProblemId) -> Result<Problem, GeneralError> {
        let record = self
            .db
            .first::<ProblemRecord, _>(
                problems::table
                    .filter(problems::id.eq(problem_id.0))
                    .filter(problems::deleted_at.is_not_null()),
            )
            .await?;
        Ok(record.to_model())
    }
}

use crate::domain::model::*;
use crate::infra::*;
use crate::schema::*;
use crate::wrapper::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::replace_into;
use std::str::FromStr;

#[derive(Clone)]
pub struct SubmitRepository {
    db: DBConnector,
}

#[derive(Clone, Queryable, Insertable)]
#[table_name = "submits"]
pub struct SubmitRecord {
    pub id: i64,
    pub user_id: i32,
    pub problem_id: i64,
    pub path: String,
    pub status: String,
    pub point: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_error: Option<String>,
    pub lang: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl SubmitRecord {
    pub fn from_model(submit: Submit) -> SubmitRecord {
        SubmitRecord {
            id: submit.id.0,
            user_id: submit.user_id.0 as i32,
            problem_id: submit.problem_id.0,
            path: submit.path,
            status: submit.status.to_string(),
            point: submit.point,
            execution_time: submit.execution_time,
            execution_memory: submit.execution_memory,
            compile_error: submit.compile_error,
            lang: submit.lang,
            created_at: submit.created_at,
            updated_at: submit.updated_at,
            deleted_at: submit.deleted_at,
        }
    }
    pub fn to_model(self) -> Submit {
        Submit {
            id: SubmitId(self.id),
            user_id: UserId(self.user_id as i64),
            problem_id: ProblemId(self.problem_id),
            path: self.path,
            status: Status::from_str(&self.status).unwrap_or(Status::IE),
            point: self.point,
            execution_time: self.execution_time,
            execution_memory: self.execution_memory,
            compile_error: self.compile_error,
            lang: self.lang,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[async_trait]
pub trait ISubmitRepository {
    fn new(db: DBConnector) -> Self;
    async fn find_by_id(&self, submit_id: &SubmitId) -> Result<Submit, GeneralError>;
    async fn pop_queing_submit(&self) -> Result<Submit, GeneralError>;
    async fn save(&self, submit: Submit) -> Result<(), GeneralError>;
}

#[async_trait]
impl ISubmitRepository for SubmitRepository {
    fn new(db: DBConnector) -> Self {
        Self { db }
    }
    async fn find_by_id(&self, submit_id: &SubmitId) -> Result<Submit, GeneralError> {
        Ok(self
            .db
            .first::<SubmitRecord, _>(submits::table.filter(submits::id.eq(submit_id.0)))
            .await?
            .to_model())
    }
    async fn pop_queing_submit(&self) -> Result<Submit, GeneralError> {
        let record = self
            .db
            .first::<SubmitRecord, _>(
                submits::table
                    .filter(submits::status.eq("WJ"))
                    .filter(submits::status.eq("WR"))
                    .filter(submits::deleted_at.is_not_null())
                    .order_by(submits::updated_at.asc()),
            )
            .await?;
        Ok(record.to_model())
    }
    async fn save(&self, submit: Submit) -> Result<(), GeneralError> {
        self.db
            .execute(
                replace_into(submits::table)
                    .values::<SubmitRecord>(SubmitRecord::from_model(submit)),
            )
            .await?;
        Ok(())
    }
}

/*
let submits = sqlx::query_as(
    r#"
    SELECT
        id
        , user_id
        , problem_id
        , path
        , status
        , point
        , execution_time
        , execution_memory
        , compile_error
        , lang
        , created_at
        , updated_at
        , deleted_at
    FROM
        submits
    WHERE
        AND deleted_at IS NULL
    ORDER BY
        updated_at ASC
    LIMIT
        1
    FOR UPDATE
    "#,
)
.fetch_one(self)
.await?;

Ok(submits)
*/

/*
async fn update_status(&mut self, id: i64, status: &str) -> Result<u64, GeneralError> {
    let result = sqlx::query!(
        r#"
        UPDATE submits
        SET
            status = ?
        WHERE
            id = ?
        "#,
        status,
        id,
    )
    .execute(self)
    .await?;
    Ok(result.rows_affected())
    */

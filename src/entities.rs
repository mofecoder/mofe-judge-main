use chrono::NaiveDateTime;
use sqlx::FromRow;
#[derive(Debug, FromRow)]
pub struct Submit {
    pub id: i64,
    pub user_id: i64,
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

#[derive(Debug, FromRow)]
pub struct Problem {
    pub id: i64,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub contest_id: Option<i64>,
    pub writer_user_id: i64,
    pub position: Option<String>,
    pub uuid: Option<String>,
    pub difficulty: String,
    pub statement: String,
    pub constraints: String,
    pub input_format: String,
    pub output_format: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow)]
pub struct Testcase {
    pub id: i64,
    pub problem_id: i64,
    pub name: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub explanation: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

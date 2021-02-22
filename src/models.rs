use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(Debug, sqlx::FromRow)]
pub struct Submit {
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

#[derive(Debug, sqlx::FromRow)]
pub struct TestcaseSets {
    pub id: i64,
    pub point: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TestcaseTestcaseSets {
    pub testcase_id: i64,
    pub testcase_set_id: i64,
}

#[derive(Debug)]
pub struct SubmissionResult {
    pub status: String,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub point: i32,
    pub compile_error: String,
    pub testcase_result_map: HashMap<i64, TestCaseResult>,
}

#[derive(Debug)]
pub struct TestCaseResult {
    pub submit_id: i64,
    pub testcase_id: i64,
    pub status: String,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/*
#[derive(Debug)]
pub struct Submits {
    pub id: i64,
    pub user_id: i32,
    pub problem_id: i64,
    pub path: String,
    pub status: String,
    pub point: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_error: String,
    pub lang: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
*/

/*
#[derive(Insertable)]
#[table_name="submits"]
pub struct NewSubmits {
    pub status: String,
    pub point: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_error: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
*/

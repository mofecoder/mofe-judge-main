use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestJson {
    pub submit_id: i64,
    pub cmd: String,              // コンパイルコマンド or 実行コマンド
    pub time_limit: i32,          // 実行制限時間
    pub mem_limit: i32,           // メモリ制限
    pub testcases: Vec<Testcase>, // pub testcase: Testcase,
    pub problem: Problem,         // pub problem: Problem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CmdResultJson {
    pub session_id: String,
    pub time: usize,
    pub result: bool,
    pub message: String,
    pub mem_usage: usize,
    pub stdout_size: usize,
    pub timeout: bool,
    pub testcase_result: TestcaseResult,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
pub struct TestcaseResult {
    pub submit_id: usize,
    pub testcase_id: usize,
    pub status: String,
    pub execution_time: usize,
    pub execution_memory: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Testcase {
    pub testcase_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Problem {
    pub problem_id: i64,
    pub uuid: String,
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

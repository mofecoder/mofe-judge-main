use crate::domain::model::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fmt::Debug};
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submit {
    pub id: SubmitId,
    pub user_id: UserId,
    pub problem_id: ProblemId,
    pub path: String,
    pub status: Status,
    pub point: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_error: Option<String>,
    pub lang: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub id: ProblemId,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub contest_id: Option<ContestId>,
    pub writer_user_id: UserId,
    pub position: Option<String>,
    pub uuid: Option<Uuid>,
    pub difficulty: String,
    pub execution_time_limit: i32,
    pub statement: String,
    pub constraints: String,
    pub input_format: String,
    pub output_format: String,
    pub checker_path: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Testcase {
    pub id: TestcaseId,
    pub problem_id: ProblemId,
    pub name: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub explanation: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TestcaseResult {
    pub status: Status,
    pub cmd_result: CmdResult,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CmdResult {
    pub execution_time: i32,   // ms
    pub stdout_size: usize,    // byte
    pub execution_memory: i32, // KB
    pub ok: bool,              // exit_code == 0
    pub message: String,       // コンパイルメッセージ
}

#[allow(dead_code)]
pub struct TestcaseSets {
    pub id: TestcaseSetsId,
    pub points: i64,
}

#[allow(dead_code)]
pub struct TestcaseTestcaseSets {
    pub testcase_id: TestcaseId,
    pub testcase_set_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompileRequest {
    pub submit_id: SubmitId,
    pub cmd: String, // コンパイルコマンド or 実行コマンド
}

#[derive(Serialize, Deserialize)]
pub struct CompileResponse(pub CmdResult);

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadRequest {
    pub submit_id: SubmitId,
    pub code_path: String, // gcp 上のパス
    pub filename: String,  // Main.ext
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JudgeRequest {
    pub submit_id: SubmitId,
    pub cmd: String,              // コンパイルコマンド or 実行コマンド
    pub time_limit: i32,          // 実行制限時間
    pub mem_limit: i32,           // メモリ制限
    pub testcases: Vec<Testcase>, // pub testcase: Testcase,
    pub problem: Problem,         // pub problem: Problem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JudgeResponse {
    pub submit_id: SubmitId,
    pub status: Status,
    pub score: i64,
    pub execution_time: i32,
    pub execution_memory: i32,
    pub testcase_result_map: HashMap<i64, TestcaseResult>,
}

#[allow(clippy::unknown_clippy_lints)]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, EnumString)]
pub enum Status {
    AC,
    TLE,
    MLE,
    OLE,
    WA,
    RE,
    CE,
    IE,
    WR,
    WIP,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Status::AC => "AC",
            Status::TLE => "TLE",
            Status::MLE => "MLE",
            Status::OLE => "OLE",
            Status::WA => "WA",
            Status::RE => "RE",
            Status::CE => "CE",
            Status::IE => "IE",
            Status::WR => "WR",
            Status::WIP => "WIP",
        };

        write!(f, "{}", s)
    }
}

impl Status {
    #[allow(dead_code)]
    pub fn to_priority(&self) -> i32 {
        match *self {
            Status::AC => 1,
            Status::TLE => 2,
            Status::MLE => 3,
            Status::OLE => 4,
            Status::WA => 5,
            Status::RE => 6,
            Status::CE => 7,
            Status::IE => 8,
            Status::WR => 9,
            Status::WIP => 10,
        }
    }
}

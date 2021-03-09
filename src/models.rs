use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileRequest {
    pub submit_id: i64,
    pub cmd: String, // コンパイルコマンド or 実行コマンド
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResponse {
    pub time: i32,          // ms
    pub stdout_size: usize, // byte
    pub mem_usage: i32,     // byte
    pub ok: bool,           // exit_code == 0
    pub message: String,    // コンパイルメッセージ
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub submit_id: i64,
    pub code_path: String, // gcp 上のパス
    pub filename: String,  // Main.ext
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JudgeRequest {
    pub submit_id: i64,
    pub cmd: String,              // コンパイルコマンド or 実行コマンド
    pub time_limit: i32,          // 実行制限時間
    pub mem_limit: i32,           // メモリ制限
    pub testcases: Vec<Testcase>, // pub testcase: Testcase,
    pub problem: Problem,         // pub problem: Problem,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JudgeResponse(Vec<TestcaseResult>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestcaseResult {
    pub status: Status,
    pub cmd_result: CmdResult,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Status {
    AC,
    TLE,
    MLE,
    OLE,
    WA,
    RE,
    CE,
    IE,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CmdResult {
    pub time: i32,          // ms
    pub stdout_size: usize, // byte
    pub mem_usage: i32,     // byte
    pub ok: bool,           // exit_code == 0
    pub message: String,    // コンパイルメッセージ
}

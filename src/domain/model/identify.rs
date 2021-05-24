use serde::*;
use Default;
#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Queryable)]
pub struct ProblemId(pub i64);
#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Queryable)]
pub struct SubmitId(pub i64);
#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct TestcaseId(pub i64);
#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct TestcaseResultId(pub i64);
#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct TestcaseSetsId(pub i64);
#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct ContestId(pub i64);
#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Queryable)]
pub struct UserId(pub i64);

use super::{
    db,
    error::Error,
    models::{Submits, TestcaseSets, TestcaseTestcaseSets},
};
use std::collections::HashMap;

#[allow(dead_code)]
pub async fn scoring(submit: Submits) -> Result<i64, Error> {
    if submit.status == "IE".to_string() || submit.status == "CE".to_string() {
        return Ok(0);
    }
    let pool = db::new_pool().await?;
    let testcase_sets: Vec<TestcaseSets> = sqlx::query_as(
        r#"
SELECT id, points FROM testcase_sets
WHERE deleted_at IS NULL AND problem_id = $1
"#,
    )
    .bind(submit.problem_id)
    .fetch_all(&pool)
    .await?;
    let testcase_testcase_sets: Vec<TestcaseTestcaseSets> = sqlx::query_as(
        r#"
SELECT testcase_id, testcase_set_id FROM testcase_testcase_sets
WHERE problem_id = $1 AND testcase
"#,
    )
    .fetch_all(&pool)
    .await?;

    let mut testcase_set_map: HashMap<i64, Vec<TestcaseTestcaseSets>> = HashMap::new();
    for test_case_set in &testcase_sets {
        testcase_set_map.insert(test_case_set.id, Vec::new());
    }
    for testcase_testcase_set in testcase_testcase_sets {}

    let mut score: i64 = 0;
    for testcase_set in &testcase_sets {
        #[allow(non_snake_case)]
        let mut is_AC = true;
        for testcase_id in &testcase_set_map[&testcase_set.id] {}
        if is_AC {
            let point: i64 = From::from(testcase_set.point);
            score += point;
        }
    }

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_status_ie() {
        let submit = Submits {
            id: 0,
            user_id: 0,
            problem_id: 0,
            path: "path".to_string(),
            status: "IE".to_string(),
            point: Some(0),
            execution_time: Some(0),
            execution_memory: Some(0),
            compile_error: None,
            lang: "Rust".to_string(),
            created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 2, 3),
            updated_at: NaiveDate::from_ymd(2020, 1, 2).and_hms(1, 2, 3),
            deleted_at: None,
        };
        let expected: i64 = 0;
        assert_eq!(Some(expected), scoring(submit).await.ok());
    }

    #[tokio::test]
    async fn test_status_ce() {
        let submit = Submits {
            id: 0,
            user_id: 0,
            problem_id: 0,
            path: "path".to_string(),
            status: "CE".to_string(),
            point: Some(0),
            execution_time: Some(0),
            execution_memory: Some(0),
            compile_error: None,
            lang: "Rust".to_string(),
            created_at: NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 2, 3),
            updated_at: NaiveDate::from_ymd(2020, 1, 2).and_hms(1, 2, 3),
            deleted_at: None,
        };
        let expected: i64 = 0;
        assert_eq!(Some(expected), scoring(submit).await.ok());
    }
}

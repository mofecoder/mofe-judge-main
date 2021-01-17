use super::{
    db,
    error::Error,
    models::{Submits, TestcaseSets, TestcaseTestcaseSets},
};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub async fn scoring(submit: Submits) -> Result<i64, Error> {
    if submit.status == "IE".to_string() || submit.status == "CE".to_string() {
        return Ok(0);
    }
    // TODO
    // pool の取り方が変わるからそれを実装
    let pool = db::new_pool().await?;
    let testcase_sets: Vec<TestcaseSets> = sqlx::query_as(
        r#"
SELECT id, points FROM testcase_sets
WHERE deleted_at IS NULL AND problem_id = ?
"#,
    )
    .bind(submit.problem_id)
    .fetch_all(&pool)
    .await?;

    let testcase_testcase_sets: Vec<TestcaseTestcaseSets> = sqlx::query_as(
        r#"
SELECT testcase_id, testcase_set_id FROM testcase_testcase_sets
INNER JOIN testcases ON testcase_testcase_sets.testcase_id = testcases.id
WHERE problem_id = ? AND testcase_testcase_sets.deleted IS NULL AND testcases.deleted_at IS NULL
"#,
    )
    .bind(submit.problem_id)
    .fetch_all(&pool)
    .await?;

    let mut testcase_set_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for testcase_testcase_set in &testcase_testcase_sets {
        match testcase_set_map.entry(testcase_testcase_set.testcase_set_id) {
            Entry::Vacant(e) => {
                e.insert(vec![testcase_testcase_set.testcase_id]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(testcase_testcase_set.testcase_id);
            }
        }
    }

    let mut score: i64 = 0;
    for testcase_set in &testcase_sets {
        let mut is_ac = true;
        for testcase_id in &testcase_set_map[&testcase_set.id] {
            // TODO
            // testcase_id, submit.id に対応する TestCaseResult を取得して、ステータスの確認
            let result_status: (String,) = sqlx::query_as(
                "SELECT status FROM testcase_results
WHERE submit_id = ? AND testcase_id = ?",
            )
            .bind(submit.id)
            .bind(testcase_id)
            .fetch_one(&pool)
            .await?;
            if result_status.0 != "AC".to_string() {
                is_ac = false;
                break;
            }
        }
        if is_ac {
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

    #[tokio::test]
    #[ignore]
    #[allow(unused_variables)]
    async fn test_scoring() {
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
        let pool = db::new_pool().await;
        // insert test dataset

        // remove test datset
    }
}

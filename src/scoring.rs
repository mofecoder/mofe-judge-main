use crate::db::DbPool;
use crate::models::{Submit, TestcaseSets, TestcaseTestcaseSets};

use anyhow::Error;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

#[allow(unused)]
pub async fn scoring(db_conn: Arc<DbPool>, submit: Submit) -> Result<i64, Error> {
    if submit.status == *"IE" || submit.status == *"CE" {
        return Ok(0);
    }
    let db_conn = Arc::as_ref(&db_conn);
    let testcase_sets: Vec<TestcaseSets> = sqlx::query_as(
        r#"
SELECT id, points FROM testcase_sets
WHERE deleted_at IS NULL AND problem_id = ?
"#,
    )
    .bind(submit.problem_id)
    .fetch_all(db_conn)
    .await?;

    let testcase_testcase_sets: Vec<TestcaseTestcaseSets> = sqlx::query_as(
        r#"
SELECT testcase_id, testcase_set_id FROM testcase_testcase_sets
INNER JOIN testcases ON testcase_testcase_sets.testcase_id = testcases.id
WHERE problem_id = ? AND testcase_testcase_sets.deleted IS NULL AND testcases.deleted_at IS NULL
"#,
    )
    .bind(submit.problem_id)
    .fetch_all(db_conn)
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
            let result_status: (String,) = sqlx::query_as(
                "SELECT status FROM testcase_results
WHERE submit_id = ? AND testcase_id = ?",
            )
            .bind(submit.id)
            .bind(testcase_id)
            .fetch_one(db_conn)
            .await?;
            if result_status.0 != *"AC" {
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

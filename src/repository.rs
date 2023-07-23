use crate::entities::Problem;
use crate::entities::{Submission, Testcase};
use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;
use sqlx::{Acquire, MySqlConnection};

#[async_trait]
pub trait SubmissionRepository: Sized {
    async fn get_submissions(&mut self) -> Result<Submission>;
    async fn update_status(&mut self, id: i64, status: &str) -> Result<u64>;
}

pub struct CafeCoderDb {}

impl CafeCoderDb {
    pub async fn get_submission(conn: &mut MySqlConnection) -> Result<Submission> {
        let submissions = sqlx::query_as(
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
                submissions
            WHERE
                (status = 'WJ' OR status = 'WR')
                AND deleted_at IS NULL
            ORDER BY
                updated_at ASC
            LIMIT
                1
            FOR UPDATE
            "#,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(submissions)
    }

    pub async fn update_status(conn: &mut MySqlConnection, id: i64, status: &str) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE submissions
            SET
                status = ?
            WHERE
                id = ?
            "#,
            status,
            id,
        )
        .execute(&mut *conn)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn fetch_problem(conn: &mut MySqlConnection, problem_id: i64) -> Result<Problem> {
        let conn = conn.acquire().await?;

        let problems = sqlx::query_as!(
            Problem,
            r#"
                SELECT
                    id
                    , slug
                    , name
                    , contest_id
                    , writer_user_id
                    , position
                    , uuid
                    , difficulty
                    , `statement`
                    , `constraints`
                    , input_format
                    , output_format
                    , created_at
                    , updated_at
                    , deleted_at
                    , checker_path
                    , execution_time_limit
                FROM
                    problems
                WHERE
                    id = ?
                    AND deleted_at IS NULL
            "#,
            problem_id,
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(problems)
    }

    pub async fn fetch_testcases(
        conn: &mut MySqlConnection,
        problem_id: i64,
    ) -> Result<Vec<Testcase>> {
        let conn = conn.acquire().await?;

        let testcases = sqlx::query_as!(
            Testcase,
            r#"
            SELECT
                id
                , problem_id
                , name
                , input
                , output
                , explanation
                , created_at
                , updated_at
                , deleted_at
            FROM
                testcases
            WHERE
                problem_id = ?
                AND deleted_at IS NULL
            "#,
            problem_id,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(testcases)
    }

    pub async fn delete_testcase_results(conn: &mut MySqlConnection, submit_id: i64) -> Result<()> {
        let conn = conn.acquire().await?;

        sqlx::query(
            r#"
            UPDATE
                testcase_results
            SET
                deleted_at = ?
            WHERE
                submission_id = ?
                AND deleted_at IS NULL
            "#,
        )
        .bind(Local::now().naive_local())
        .bind(submit_id)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

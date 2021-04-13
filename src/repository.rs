use crate::entities::{Submit, Testcase};
use crate::{db::DbPool, entities::Problem};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{MySql, Transaction};
use chrono::prelude::*;

#[async_trait]
pub trait SubmitRepository {
    async fn get_submits(&mut self) -> Result<Submit>;
    async fn update_status(&mut self, id: i64, status: &str) -> Result<u64>;
}

#[async_trait]
impl SubmitRepository for Transaction<'_, MySql> {
    async fn get_submits(&mut self) -> Result<Submit> {
        let submits = sqlx::query_as(
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
                submits 
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
        .fetch_one(self)
        .await?;

        Ok(submits)
    }

    async fn update_status(&mut self, id: i64, status: &str) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE submits 
            SET
                status = ? 
            WHERE
                id = ? 
            "#,
            status,
            id,
        )
        .execute(self)
        .await?;
        Ok(result.rows_affected())
    }
}

#[async_trait]
pub trait ProblemsRepository {
    async fn fetch_problem(&self, problem_id: i64) -> Result<Problem>;
}
#[async_trait]
impl ProblemsRepository for DbPool {
    async fn fetch_problem(&self, problem_id: i64) -> Result<Problem> {
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
        .fetch_one(self)
        .await?;
        Ok(problems)
    }
}

#[async_trait]
pub trait TestcasesRepository {
    async fn fetch_testcases(&self, problem_id: i64) -> Result<Vec<Testcase>>;
}
#[async_trait]
impl TestcasesRepository for DbPool {
    async fn fetch_testcases(&self, problem_id: i64) -> Result<Vec<Testcase>> {
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
        .fetch_all(self)
        .await?;
        Ok(testcases)
    }
}

#[async_trait]
pub trait TestcaseResultsRepository {
    async fn delete_testcase_results(&self, submit_id: i64) -> Result<()>;
}
#[async_trait]
impl TestcaseResultsRepository for DbPool {
    async fn delete_testcase_results(&self, submit_id: i64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE 
                testcase_results
            SET 
                deleted_at = ?
            WHERE 
                submit_id = ? 
                AND deleted_at IS NULL
            "#,
        )
        .bind(Local::now().naive_local())
        .bind(submit_id)
        .execute(self)
        .await?;
    
        Ok(())
    }
}
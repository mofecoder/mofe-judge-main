use crate::error::*;
use crate::infra::connection_pool::MySQLConnPool;

#[derive(Debug)]
pub enum DBExecutorError {
    DBError,
    RecordNotFound,
}

impl IGeneralError for DBExecutorError {
    fn error_type(&self) -> String {
        use DBExecutorError::*;

        match self {
            DBError => "db_error",
            RecordNotFound => "record_not_found",
        }
        .to_string()
    }
}

impl From<diesel::result::Error> for GeneralError {
    fn from(err: diesel::result::Error) -> GeneralError {
        use diesel::result::Error::*;

        match err {
            NotFound => GeneralError::new(DBExecutorError::RecordNotFound, err),
            _ => GeneralError::new(DBExecutorError::DBError, err),
        }
    }
}

#[derive(Clone)]
pub struct DBExecutor(MySQLConnPool);

impl DBExecutor {
    pub fn new(database_url: String, size_conn_pool: u32) -> DBExecutor {
        DBExecutor(MySQLConnPool::new(database_url, size_conn_pool))
    }

    pub fn get_connection(
        &self,
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>> {
        self.0.get_connection()
    }
}

#[derive(Clone)]
pub struct DBConnector(DBExecutor);

impl DBConnector {
    pub fn new(executor: DBExecutor) -> DBConnector {
        DBConnector(executor)
    }

    #[allow(dead_code)]
    pub async fn ensure_initialized(&self) -> Result<(), GeneralError> {
        let executor = self.0.clone();

        tokio::task::spawn_blocking(move || {
            executor.0.ensure_initialized();
            Ok(())
        })
        .await?
    }

    pub async fn execute<Q: Send + 'static>(&self, query: Q) -> Result<usize, GeneralError>
    where
        Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
        Q: diesel::query_builder::QueryFragment<diesel::mysql::Mysql>,
        Q: diesel::query_builder::QueryId,
    {
        let conn = self.0.get_connection();

        tokio::task::spawn_blocking(move || {
            let result = query.execute(&conn)?;
            Ok(result)
        })
        .await?
    }

    pub async fn first<T: 'static + Send, Q: 'static + Send>(
        &self,
        query: Q,
    ) -> Result<T, GeneralError>
    where
        Q: diesel::query_dsl::limit_dsl::LimitDsl,
        Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
        diesel::helper_types::Limit<Q>: diesel::query_dsl::LoadQuery<diesel::MysqlConnection, T>,
    {
        let conn = self.0.get_connection();

        tokio::task::spawn_blocking(move || {
            let result = query.first(&conn)?;
            Ok(result)
        })
        .await?
    }

    pub async fn load<T: 'static + Send, Q: 'static + Send>(
        &self,
        query: Q,
    ) -> Result<Vec<T>, GeneralError>
    where
        Q: diesel::RunQueryDsl<diesel::MysqlConnection>,
        Q: diesel::query_dsl::LoadQuery<diesel::MysqlConnection, T>,
    {
        let conn = self.0.get_connection();

        tokio::task::spawn_blocking(move || {
            let result = query.load(&conn)?;
            Ok(result)
        })
        .await?
    }

    #[allow(dead_code)]
    pub async fn load_sql<
        T: 'static + Send + diesel::deserialize::QueryableByName<diesel::mysql::Mysql>,
    >(
        &self,
        query: impl Into<String>,
    ) -> Result<Vec<T>, GeneralError> {
        let conn = self.0.get_connection();
        let q = query.into();

        tokio::task::spawn_blocking(move || {
            use diesel::prelude::*;

            let result = diesel::sql_query(q).load::<T>(&conn)?;
            Ok(result)
        })
        .await?
    }
}

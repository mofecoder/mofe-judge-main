#[derive(Debug)]
pub enum Error {
    SqlxError { e: sqlx::Error },
    BollardError { e: bollard::errors::Error },
    TokioJoinError { e: tokio::task::JoinError },
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::SqlxError { e }
    }
}

impl From<bollard::errors::Error> for Error {
    fn from(e: bollard::errors::Error) -> Self {
        Error::BollardError { e }
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Error::TokioJoinError { e }
    }
}

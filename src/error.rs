#[derive(Debug)]
pub enum Error {
    SqlxError { e: sqlx::Error },
    BollardError { e: bollard::errors::Error },
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

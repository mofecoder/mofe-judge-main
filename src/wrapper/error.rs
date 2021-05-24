use anyhow::Error;
use std::any::Any;

pub trait IGeneralError: Any {
    fn error_type(&self) -> String {
        "internal_server_error".to_string()
    }
}

#[derive(Debug)]
pub struct GeneralError {
    type_id: std::any::TypeId,
    error_type: String,
    inner: Error,
}

impl GeneralError {
    pub fn new<E>(err: impl IGeneralError, detail: E) -> GeneralError
    where
        Error: From<E>,
    {
        GeneralError {
            type_id: err.type_id(),
            error_type: err.error_type(),
            inner: From::from(detail),
        }
    }

    pub fn into_inner(self) -> Error {
        self.inner
    }

    pub fn error_type(&self) -> String {
        self.error_type.clone()
    }

    pub fn is_error_of(&self, err: impl IGeneralError) -> bool {
        self.type_id == err.type_id() && self.error_type() == err.error_type()
    }
    pub fn only(msg: &'static str) -> GeneralError {
        GeneralError {
            type_id: std::any::TypeId::of::<GeneralError>(),
            error_type: String::from(msg),
            inner: Error::msg(msg.clone()),
        }
    }
}

// failure::Error can be treated as GeneralError
impl IGeneralError for Error {}

pub enum FutureError {
    JoinError,
}
pub enum DockerError {
    InternalError,
}

// for tokio::task::spawn_blocking
impl IGeneralError for FutureError {
    fn error_type(&self) -> String {
        match self {
            FutureError::JoinError => "internal_server_error".to_string(),
        }
    }
}
impl IGeneralError for DockerError {
    fn error_type(&self) -> String {
        match self {
            DockerError::InternalError => "internal_docker_error".to_string(),
        }
    }
}
impl IGeneralError for bollard::errors::Error {
    fn error_type(&self) -> String {
        "docker_error".to_string()
    }
}

impl From<tokio::task::JoinError> for GeneralError {
    fn from(err: tokio::task::JoinError) -> GeneralError {
        GeneralError::new(FutureError::JoinError, err)
    }
}

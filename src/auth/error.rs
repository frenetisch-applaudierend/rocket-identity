use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("The user is not authenticated")]
    Unauthenticated,

    #[error("The user is authenticated but has not passed the policy")]
    PolicyFailed,

    #[error("Some other error happened")]
    Other(Box<dyn std::error::Error>),
}

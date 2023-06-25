use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("The user could not be found")]
    UserNotFound,

    #[error("The user could be found but has no password")]
    MissingPassword,

    #[error("The provided password does not match the users")]
    IncorrectPassword,

    #[error("Some other error happened")]
    Other(Box<dyn std::error::Error>),
}

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("The user is not authenticated")]
    Unauthenticated,

    #[error("The user is authenticated but has not passed the policy")]
    PolicyFailed,

    #[error("Some other error happened")]
    Other(Box<dyn std::error::Error>),
}

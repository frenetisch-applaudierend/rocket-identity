use crate::util::AnyError;

use super::scheme::AuthenticationError;

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("The user could not be found")]
    UserNotFound,

    #[error("The user could be found but has no password")]
    MissingPassword,

    #[error("The provided password does not match the users")]
    IncorrectPassword,

    #[error("Some other error happened")]
    Other(AnyError),
}

impl From<LoginError> for AuthenticationError {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::UserNotFound => AuthenticationError::Unauthenticated,
            LoginError::MissingPassword => AuthenticationError::Unauthenticated,
            LoginError::IncorrectPassword => AuthenticationError::Unauthenticated,
            LoginError::Other(err) => AuthenticationError::Other(Some(err)),
        }
    }
}
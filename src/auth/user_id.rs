#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new_uuid() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl TryFrom<&str> for UserId {
    type Error = UserIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(UserIdError::Empty)
        } else {
            Ok(Self(value.to_owned()))
        }
    }
}

impl TryFrom<String> for UserId {
    type Error = UserIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(UserIdError::Empty)
        } else {
            Ok(Self(value))
        }
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserIdError {
    #[error("UserId cannot be empty")]
    Empty,
}

pub type Result<T> = std::result::Result<T, BoxError>;

pub type BoxError = Box<dyn std::error::Error>;

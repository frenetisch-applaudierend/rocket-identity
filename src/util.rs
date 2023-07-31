pub type Result<T> = std::result::Result<T, BoxError>;

pub type BoxError = Box<dyn std::error::Error>;

pub trait BoxableError: std::error::Error {
    fn boxed(self) -> BoxError
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<T: std::error::Error> BoxableError for T {}

pub type Result<T> = std::result::Result<T, AnyError>;

pub trait ClonableError: std::error::Error {
    fn dyn_clone(&self) -> AnyError;
}

impl<T> ClonableError for T
where
    T: std::error::Error + Clone + Send + Sync + 'static,
{
    fn dyn_clone(&self) -> AnyError {
        <Self as Clone>::clone(self).boxed()
    }
}

pub type AnyError = Box<dyn ClonableError + Send + Sync>;

impl<T> From<T> for AnyError
where
    T: std::error::Error + Clone + Send + Sync + 'static,
{
    fn from(err: T) -> Self {
        err.dyn_clone()
    }
}

pub(crate) trait Boxable {
    fn boxed(self) -> Box<Self>;
}

impl<T> Boxable for T
where
    T: 'static,
{
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

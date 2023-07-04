pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

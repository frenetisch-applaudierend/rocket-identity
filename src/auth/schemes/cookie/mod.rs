mod scheme;

pub use scheme::*;

pub fn default() -> CookieScheme {
    CookieScheme::default()
}

pub fn with_name(name: &str) -> CookieScheme {
    CookieScheme::new(name.to_string())
}

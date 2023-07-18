mod auth;
mod fairing;
mod persistence;
mod services;

pub mod config;
pub mod util;

pub use auth::*;
pub use fairing::*;
pub use persistence::*;
pub use services::*;

pub struct Identity {
    config: tokio::sync::RwLock<Option<config::Config>>,
}

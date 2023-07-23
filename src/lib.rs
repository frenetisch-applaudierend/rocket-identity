mod auth;
mod fairing;
mod services;

pub mod config;
pub mod stores;
pub mod util;
pub mod schemes;

pub use auth::*;
pub use fairing::*;
pub use services::*;

pub struct Identity {
    config: tokio::sync::RwLock<Option<config::Config>>,
}

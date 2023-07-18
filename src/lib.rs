pub mod config;
pub mod persistence;
pub mod util;

mod auth;
mod fairing;
mod services;

pub use auth::*;
pub use fairing::*;
pub use services::*;

pub struct Identity {
    config: tokio::sync::RwLock<Option<config::Config>>,
}

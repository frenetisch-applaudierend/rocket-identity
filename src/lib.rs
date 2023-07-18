pub mod auth;
pub mod config;
pub mod persistence;
pub mod util;

mod fairing;
mod services;

pub use fairing::*;
pub use services::*;

pub struct Identity {
    config: tokio::sync::RwLock<Option<config::Config>>,
}

pub mod auth;
pub mod config;
pub mod persistence;
pub mod util;

mod fairing;

pub use fairing::*;

pub struct Identity {
    config: tokio::sync::RwLock<Option<config::Config>>,
}

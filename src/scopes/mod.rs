//! src/routes/mod.rs
pub mod api;
pub mod auth;
pub mod health_check;
pub mod oauth;

pub use api::*;
pub use auth::*;
pub use health_check::*;
pub use oauth::*;

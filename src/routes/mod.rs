//! src/routes/mod.rs
mod games;
mod health_check;
pub mod oauth;

pub use games::*;
pub use health_check::*;
pub use oauth::*;

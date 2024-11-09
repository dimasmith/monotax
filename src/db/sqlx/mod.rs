//! Save and retrieve data from sqlite.
//!
//! The module implements domain repositories and provides means to initialize and migrate monotax
//! database.
pub mod connection;
mod criteria;
mod income_repository;
pub mod init;
mod payment_repository;
mod record;
mod tax_payment_repository;

pub use income_repository::income_repository;
pub use payment_repository::payment_repository;
pub use tax_payment_repository::payment_tax_repository;

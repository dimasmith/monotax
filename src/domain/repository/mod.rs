//! Repository traits and definitions.

pub mod income;
pub mod payment;
pub mod tax_payment;

pub use income::IncomeRepository;
pub use payment::PaymentRepository;
pub use tax_payment::TaxPaymentRepository;

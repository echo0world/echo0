//! # Chisel
//!
//! Zero-dependency CPI helpers for Solana programs.
//!
//! Chisel provides lightweight wrappers for cross-program invocations
//! to common Solana programs without importing their full crates.

#![no_std]

pub mod system;
pub mod token;
#[cfg(feature = "token-2022")]
pub mod token_2022;
pub mod associated_token;
pub mod invoke;
pub mod program_ids;
pub mod error;

pub use error::ChiselError;
pub type ChiselResult = Result<(), ChiselError>;

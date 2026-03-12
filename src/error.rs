//! Minimal error types for Chisel.

/// Errors that can occur during CPI invocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChiselError {
    /// The CPI invoke call failed.
    InvokeFailed,
    /// Invalid account data.
    InvalidAccountData,
    /// Missing required signer.
    MissingSigner,
    /// Insufficient funds.
    InsufficientFunds,
}

// feat add InvalidInstruction error variant

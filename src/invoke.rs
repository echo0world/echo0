//! Raw invoke wrappers using Solana syscalls.
//!
//! These directly call the `sol_invoke_signed_c` syscall with zero overhead.

/// Invoke a cross-program instruction.
///
/// This directly calls the sol_invoke_signed syscall with no signer seeds.
#[inline(always)]
pub fn invoke(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
) -> Result<(), u64> {
    invoke_signed(instruction, account_infos, &[])
}

/// Invoke a cross-program instruction with PDA signer seeds.
#[inline(always)]
pub fn invoke_signed(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]],
) -> Result<(), u64> {
    #[cfg(target_os = "solana")]
    {
        extern "C" {
            fn sol_invoke_signed_c(
                instruction_addr: *const u8,
                account_infos_addr: *const u8,
                account_infos_len: u64,
                signers_seeds_addr: *const u8,
                signers_seeds_len: u64,
            ) -> u64;
        }

        let result = unsafe {
            sol_invoke_signed_c(
                instruction as *const Instruction as *const u8,
                account_infos.as_ptr() as *const u8,
                account_infos.len() as u64,
                signer_seeds.as_ptr() as *const u8,
                signer_seeds.len() as u64,
            )
        };

        if result != 0 {
            return Err(result);
        }
    }

    Ok(())
}

/// Minimal instruction representation for CPI.
///
/// Matches the layout expected by the Solana runtime's `sol_invoke_signed_c` syscall.
#[repr(C)]
pub struct Instruction {
    /// Pointer to the 32-byte program ID.
    pub program_id: *const u8,
    /// Pointer to the array of account metas.
    pub accounts: *const AccountMeta,
    /// Number of account metas.
    pub accounts_len: u64,
    /// Pointer to the instruction data.
    pub data: *const u8,
    /// Length of the instruction data.
    pub data_len: u64,
}

/// Account metadata for CPI instructions.
#[repr(C)]
pub struct AccountMeta {
    /// Pointer to the 32-byte public key.
    pub pubkey: *const u8,
    /// Whether this account must sign the transaction.
    pub is_signer: bool,
    /// Whether this account's data may be mutated.
    pub is_writable: bool,
}

/// Opaque account info handle — passed through from the runtime.
///
/// This is the raw pointer the runtime gives us. We never dereference it
/// ourselves; it's only passed back to the runtime via CPI.
#[repr(C)]
pub struct AccountInfo {
    _opaque: [u8; 0],
}

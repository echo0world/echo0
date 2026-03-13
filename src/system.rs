//! System Program CPI helpers.
//!
//! Wrappers for common System Program instructions with zero dependencies.

use crate::invoke::{invoke, invoke_signed, Instruction, AccountMeta, AccountInfo};
use crate::program_ids::SYSTEM_PROGRAM_ID;
use crate::ChiselResult;

/// Transfer lamports between accounts.
///
/// # Arguments
/// * `from` - Source account (must be signer, writable)
/// * `to` - Destination account (writable)
/// * `lamports` - Amount to transfer
#[inline(always)]
pub fn transfer(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
) -> ChiselResult {
    let mut data = [0u8; 12];
    data[0..4].copy_from_slice(&2u32.to_le_bytes());
    data[4..12].copy_from_slice(&lamports.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: from as *const _ as *const u8, is_signer: true, is_writable: true },
        AccountMeta { pubkey: to as *const _ as *const u8, is_signer: false, is_writable: true },
    ];

    let ix = Instruction {
        program_id: SYSTEM_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 2,
        data: data.as_ptr(),
        data_len: 12,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::ChiselError::InvokeFailed)
}

/// Transfer lamports with PDA signer.
#[inline(always)]
pub fn transfer_signed(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
    signer_seeds: &[&[&[u8]]],
) -> ChiselResult {
    let mut data = [0u8; 12];
    data[0..4].copy_from_slice(&2u32.to_le_bytes());
    data[4..12].copy_from_slice(&lamports.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: from as *const _ as *const u8, is_signer: true, is_writable: true },
        AccountMeta { pubkey: to as *const _ as *const u8, is_signer: false, is_writable: true },
    ];

    let ix = Instruction {
        program_id: SYSTEM_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 2,
        data: data.as_ptr(),
        data_len: 12,
    };

    invoke_signed(&ix, &[], signer_seeds).map_err(|_| crate::error::ChiselError::InvokeFailed)
}

/// Create a new account.
///
/// # Arguments
/// * `payer` - Account paying for the new account (signer, writable)
/// * `new_account` - The new account to create (signer, writable)
/// * `lamports` - Lamports to fund the new account
/// * `space` - Space in bytes for the account data
/// * `owner` - Program that will own the new account
#[inline(always)]
pub fn create_account(
    payer: &AccountInfo,
    new_account: &AccountInfo,
    lamports: u64,
    space: u64,
    owner: &[u8; 32],
) -> ChiselResult {
    let mut data = [0u8; 52];
    data[0..4].copy_from_slice(&0u32.to_le_bytes());
    data[4..12].copy_from_slice(&lamports.to_le_bytes());
    data[12..20].copy_from_slice(&space.to_le_bytes());
    data[20..52].copy_from_slice(owner);

    let accounts = [
        AccountMeta { pubkey: payer as *const _ as *const u8, is_signer: true, is_writable: true },
        AccountMeta { pubkey: new_account as *const _ as *const u8, is_signer: true, is_writable: true },
    ];

    let ix = Instruction {
        program_id: SYSTEM_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 2,
        data: data.as_ptr(),
        data_len: 52,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::ChiselError::InvokeFailed)
}

/// Allocate space for an account.
pub fn allocate(account: &AccountInfo, space: u64) -> ChiselResult {
    let mut data = [0u8; 12];
    data[0..4].copy_from_slice(&8u32.to_le_bytes());
    data[4..12].copy_from_slice(&space.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: account as *const _ as *const u8, is_signer: true, is_writable: true },
    ];

    let ix = Instruction {
        program_id: SYSTEM_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 1,
        data: data.as_ptr(),
        data_len: 12,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::ChiselError::InvokeFailed)
}

/// Assign an account to a program.
pub fn assign(account: &AccountInfo, owner: &[u8; 32]) -> ChiselResult {
    let mut data = [0u8; 36];
    data[0..4].copy_from_slice(&1u32.to_le_bytes());
    data[4..36].copy_from_slice(owner);

    let accounts = [
        AccountMeta { pubkey: account as *const _ as *const u8, is_signer: true, is_writable: true },
    ];

    let ix = Instruction {
        program_id: SYSTEM_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 1,
        data: data.as_ptr(),
        data_len: 36,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::ChiselError::InvokeFailed)
}

// sync_native wrapper

# Examples

## Basic SOL Transfer

```rust
use chisel::system;

pub fn process_transfer(
    accounts: &[AccountInfo],
    lamports: u64,
) -> ProgramResult {
    let from = &accounts[0];
    let to = &accounts[1];

    system::transfer(from, to, lamports)?;
    Ok(())
}
```

## Token Transfer with PDA Signer

A common pattern: a program-owned vault transfers tokens using its PDA as the authority.

```rust
use chisel::token;

pub fn withdraw_from_vault(
    accounts: &[AccountInfo],
    amount: u64,
    bump: u8,
) -> ProgramResult {
    let vault = &accounts[0];       // source token account
    let user_ata = &accounts[1];    // destination token account
    let authority = &accounts[2];   // PDA authority

    let seeds: &[&[u8]] = &[b"vault", &[bump]];

    token::transfer_signed(
        vault,
        user_ata,
        authority,
        amount,
        &[seeds],
    )?;

    Ok(())
}
```

## Create Account and Initialize

```rust
use chisel::system;

pub fn create_data_account(
    accounts: &[AccountInfo],
    space: u64,
    lamports: u64,
    program_id: &[u8; 32],
) -> ProgramResult {
    let payer = &accounts[0];
    let new_account = &accounts[1];

    system::create_account(
        payer,
        new_account,
        lamports,
        space,
        program_id,
    )?;

    Ok(())
}
```

## Create Associated Token Account (Idempotent)

Safe to call even if the ATA already exists.

```rust
use chisel::associated_token;

pub fn ensure_ata_exists(
    accounts: &[AccountInfo],
) -> ProgramResult {
    let payer = &accounts[0];
    let wallet = &accounts[1];
    let mint = &accounts[2];

    associated_token::create_idempotent(
        payer,
        wallet,
        mint,
    )?;

    Ok(())
}
```

## Token Mint + Transfer Pipeline

```rust
use chisel::token;

pub fn mint_and_distribute(
    accounts: &[AccountInfo],
    mint_amount: u64,
) -> ProgramResult {
    let mint = &accounts[0];
    let treasury = &accounts[1];
    let recipient = &accounts[2];
    let authority = &accounts[3];

    // Mint tokens to treasury
    token::mint_to(mint, treasury, authority, mint_amount)?;

    // Transfer half to recipient
    token::transfer(treasury, recipient, authority, mint_amount / 2)?;

    Ok(())
}
```

## Closing a Token Account

Recover the rent lamports from an empty token account.

```rust
use chisel::token;

pub fn close_empty_account(
    accounts: &[AccountInfo],
) -> ProgramResult {
    let token_account = &accounts[0];
    let rent_destination = &accounts[1];
    let authority = &accounts[2];

    token::close_account(token_account, rent_destination, authority)?;

    Ok(())
}
```

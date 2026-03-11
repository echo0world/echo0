# chisel 🪨

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange)](https://crates.io/crates/chisel-cpi)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Solana](https://img.shields.io/badge/Solana-Program%20Library-purple)](https://solana.com)

> Chisel away the bloat from your cross-program calls.

I've got no dependencies

To hold me down

To make me fret

Or make me frown

I had dependencies

But now I'm free

There are no dependencies on me

## Overview

**Chisel** is a zero-dependency CPI (Cross-Program Invocation) helper library for Solana programs in Rust. It provides lightweight, zero-copy wrappers for calling common Solana programs without importing their full crates.

Stop dragging in `spl-token`, `spl-associated-token-account`, and `solana-program` just to make a transfer. Chisel gives you clean, minimal CPI calls that compile to tight BPF bytecode.

No dependencies. No bloat. Just the invoke.

## Features

- **no_std** — no standard library required
- **Zero external dependencies** — only raw Solana syscalls
- **Zero-copy** — account data is never copied, just referenced
- **Minimal binary size** — each CPI wrapper compiles to near-optimal BPF instructions
- **Type-safe** — compile-time checks for account constraints
- **Covers the essentials:**
  - System Program (transfers, create account, allocate, assign)
  - SPL Token Program (transfer, mint_to, burn, approve, revoke, close_account, freeze/thaw)
  - Associated Token Account Program (create, create_idempotent)
  - Token-2022 / Token Extensions (transfer_checked, mint_to_checked)

## Getting started

From your project folder:

```bash
cargo add chisel-cpi
```

## Usage

### System Program — Transfer SOL

```rust
use chisel::system;

// Transfer 1 SOL from payer to recipient
system::transfer(
    payer_info,
    recipient_info,
    1_000_000_000, // lamports
)?;
```

### SPL Token — Transfer Tokens

```rust
use chisel::token;

// Transfer 100 tokens (with 6 decimals)
token::transfer(
    source_info,
    destination_info,
    authority_info,
    100_000_000,
)?;
```

### SPL Token — Transfer Checked

```rust
use chisel::token;

token::transfer_checked(
    source_info,
    mint_info,
    destination_info,
    authority_info,
    amount,
    decimals,
)?;
```

### Associated Token Account — Create

```rust
use chisel::associated_token;

// Create ATA for wallet
associated_token::create(
    payer_info,
    wallet_info,
    mint_info,
)?;
```

### Create Account with Seed

```rust
use chisel::system;

system::create_account_with_seed(
    payer_info,
    new_account_info,
    base_info,
    seed,
    lamports,
    space,
    owner,
)?;
```

### Signed Invocations (PDA Signers)

Every CPI function has a `_signed` variant for PDA-derived accounts:

```rust
use chisel::token;

let seeds: &[&[u8]] = &[b"vault", user_key.as_ref(), &[bump]];

token::transfer_signed(
    source_info,
    destination_info,
    authority_info,
    amount,
    &[seeds],
)?;
```

## Why Chisel?

| | chisel | anchor CPI | raw invoke |
|---|---|---|---|
| Dependencies | 0 | 15+ | 0 |
| Binary overhead | ~200B per CPI | ~8KB+ | ~200B |
| Type safety | ✅ | ✅ | ❌ |
| Readability | ✅ | ✅ | ❌ |
| Compute overhead | Minimal | Moderate | Minimal |

Every byte matters on-chain. Chisel gives you the ergonomics of Anchor's CPI helpers with the efficiency of hand-rolled `invoke` calls.

## Architecture

```
chisel/
├── src/
│   ├── lib.rs              # Re-exports all modules
│   ├── system.rs           # System Program CPIs
│   ├── token.rs            # SPL Token CPIs
│   ├── token_2022.rs       # Token Extensions CPIs
│   ├── associated_token.rs # ATA Program CPIs
│   ├── invoke.rs           # Raw invoke/invoke_signed wrappers
│   ├── program_ids.rs      # Compile-time program ID constants
│   └── error.rs            # Minimal error types
├── tests/
│   └── integration.rs
├── Cargo.toml
├── LICENSE
└── README.md
```

## Supported Programs

| Program | Module | Instructions |
|---|---|---|
| System Program | `chisel::system` | transfer, create_account, allocate, assign |
| SPL Token | `chisel::token` | transfer, transfer_checked, mint_to, burn, approve, revoke, close_account, freeze_account, thaw_account |
| Token-2022 | `chisel::token_2022` | transfer_checked, mint_to_checked |
| Associated Token | `chisel::associated_token` | create, create_idempotent |

## Design Principles

1. **No allocations** — everything is stack or account-buffer based
2. **No serialization libraries** — instruction data is built inline
3. **No program crate imports** — program IDs and instruction discriminators are constants
4. **Compile-time where possible** — const program IDs, const instruction indices

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT — see [LICENSE](LICENSE) for details.

<!-- improved getting started -->

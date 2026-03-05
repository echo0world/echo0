# Contributing to Chisel

Thanks for your interest in contributing!

## Getting Started

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a PR

## Guidelines

- Keep it zero-dependency. That's the whole point.
- Every CPI wrapper should have a `_signed` variant for PDA signers.
- Use `#[inline(always)]` for hot-path functions.
- No allocations. Stack only.
- Document every public function.

## Adding a New Program

1. Create `src/<program_name>.rs`
2. Add the program ID to `src/program_ids.rs`
3. Implement CPI wrappers following existing patterns
4. Add to `src/lib.rs` re-exports
5. Update README.md with usage examples

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Keep binary size in mind — every byte counts on-chain

#!/usr/bin/env bash
set -euo pipefail

# Quick pre-commit checks
echo "=> formatting..."
cargo fmt --all --check
echo "=> clippy..."
cargo clippy --all-features -- -D warnings
echo "=> tests..."
cargo test --all-features
echo "All checks passed."

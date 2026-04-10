## jup-xorca-integration

Integration of the xORCA staking program with Jupiter’s `jupiter-amm-interface`. It exposes the `Amm` implementation that can load on-chain state, derive the vault
ATA, and produce quotes for staking ORCA into xORCA.

### Usage

- Build: `cargo build`
- Tests: `cargo test`
- Lint/format: `cargo fmt && cargo clippy`

### Notes

- `rust_decimal = 1.36.0` is pinned to match `jupiter-amm-interface`.
- This crate uses the split `solana-pubkey = "3"` dependency instead of the umbrella `solana-sdk` crate.

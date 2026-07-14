# WebWallet

Educational browser-based wallet built with Rust, WebAssembly, Leptos CSR, and Trunk.

## Project Status

This project is a learning wallet, not a production wallet. The current focus is a Solana Devnet wallet flow with security-first backend milestones.

Completed backend learning areas include:

- Wallet session models
- Solana Devnet balance and airdrop RPC helpers
- Solana transfer modeling and validation
- Transaction signing boundaries
- Transaction broadcast RPC helpers
- Password, encryption key, and key derivation models
- Encrypted wallet vault primitives
- Browser-safe random salt and nonce helpers
- Encrypted browser storage

## Security Rules

This project follows strict educational wallet rules:

- Devnet/testnet only
- Never store raw private keys
- Never store raw mnemonics
- Never store raw passwords
- Never log wallet secrets
- Never send wallet secrets to a server
- Store only encrypted wallet data
- Keep decrypted wallet data only in memory
- Do not claim production readiness

## Tech Stack

- Rust stable
- Leptos CSR
- Trunk
- WebAssembly target: `wasm32-unknown-unknown`
- Solana Devnet
- `serde` / `serde_json`
- `gloo-storage`
- `wasm-bindgen` / `web-sys`

## Development

Common checks:

```bash
cargo fmt
cargo test
cargo check --target wasm32-unknown-unknown
trunk build
```

Run locally:

```bash
trunk serve
```

Then open:

```text
http://127.0.0.1:8080/
```

## Learning Goal

The goal is to understand how wallet systems are built step by step: models first, then validation, signing, encryption, storage, and finally lock/unlock flows.

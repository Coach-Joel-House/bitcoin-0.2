# bitcoin-rs-core

A Bitcoin-inspired full node implementation written **100% in Rust**.

This project aims to explore a clean, modern, and auditable architecture for a Bitcoin-style blockchain node, focusing on correctness, safety, and long-term maintainability.

> ⚠️ **Status:** Early development / research phase  
> This software is not production-ready and should not be used with real funds.

---

## Goals

- Bitcoin-style **UTXO-based ledger**
- **Nakamoto Proof-of-Work** consensus
- Fixed supply monetary model (Bitcoin-inspired)
- Clear separation of consensus-critical code
- Rust-first design emphasizing safety and correctness
- Educational and experimental value

---

## Non-Goals

- ❌ Bitcoin mainnet compatibility (for now)
- ❌ Replacing Bitcoin Core
- ❌ High-throughput / smart-contract execution
- ❌ Fast iteration at the expense of consensus safety

This project prioritizes **correctness over speed**.

---

## Architecture Overview

The repository is structured similarly to Bitcoin Core, but redesigned for Rust:

bitcoin-rs-core/
├── chain/ # Chainstate, validation, reorg logic
├── consensus/ # Proof-of-Work, difficulty, consensus rules
├── mempool/ # Transaction pool and policies
├── p2p/ # Peer-to-peer networking
├── primitives/ # Blocks, transactions, headers
├── rpc/ # RPC interface (future)
├── storage/ # Persistent storage (blocks, UTXO)
├── utxo/ # UTXO set management
├── wallet/ # Wallet logic (optional, future)
├── src/
│ └── main.rs # Node entry point
├── Cargo.toml
└── Cargo.lock


Consensus-critical logic is intentionally isolated to reduce the risk of accidental rule changes.

---

## Building & Running

### Requirements
- Rust (stable)
- Cargo

### Build
```bash
cargo build
Run
cargo run
Design Philosophy
Consensus rules are sacred
Any change to consensus logic must be explicit, reviewed, and intentional.

UTXO ownership over account balances
Ownership is defined by cryptographic proofs, not mutable accounts.

Safety > performance
A slow correct node is better than a fast broken one.

Clarity over cleverness
Readable code is preferred over micro-optimizations.

License
MIT Licens.

Disclaimer
This project is for educational and research purposes only.
It is not affiliated with Bitcoin Core or the Bitcoin project.
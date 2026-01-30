# Bitcoin RS Core

This repository contains the reference Rust implementation of the Bitcoin RS network.

The project proposes a successor to Bitcoin v0.2, preserving the original peer-to-peer and proof-of-work principles while extending the networking layer beyond the traditional model.

The network is live. Consensus rules are finalized.

---

## Design Goals

* Preserve simple, auditable consensus rules
* Require long-term commitment from participants
* Extend peer-to-peer communication beyond TCP-only models
* Support operation in constrained or partially offline environments

This implementation prioritizes correctness and clarity over feature breadth.

---

## Consensus

The consensus rules are **fixed and frozen**.

They will not be changed in the future.

Participation in the network requires **continuous mining for one full year**, serving as a commitment mechanism rather than a permission system.

Nodes that do not meet this requirement may observe the network but are not considered committed participants.

---

## Networking

The system extends peer-to-peer communication beyond the current Bitcoin model.

Planned and implemented networking layers include:

* Standard TCP-based P2P
* Offline peer-to-peer communication
* Local computer-to-computer P2P
* Geolocation-based P2P (planned)
* Bluetooth P2P (planned)
* Satellite P2P (planned)

The goal is to reduce dependency on any single transport layer.

---

## Implementation Status

* Core blockchain logic implemented in Rust
* UTXO ownership based on public key hashes
* Wallet encryption and signing supported
* Local explorer interface available
* Multi-transport networking stack under active development

A Fly.io deployment exists for convenience, but contributors are not required to use it.

---

## Development

This project follows a **code-first** development model.

Core contributors are expected to:

* Download the repository
* Run a local node
* Debug and review the code directly
* Choose their own development environment and tooling

There is no prescribed workflow. Familiarity with low-level systems programming, networking, and consensus code is assumed.

---

## Contribution

This project does not follow a feature-request-driven roadmap.

Contributions should focus on:

* Code review
* Bug fixes
* Performance and correctness improvements
* Documentation of observed behavior

Discussion related to core development takes place in the official Core group.

---

## Core Discussion

Core contributor discussion is conducted here:

[https://t.me/+FnHYI5mEAkIzODRl](https://t.me/+FnHYI5mEAkIzODRl)

---

## License

Specify the license in the appropriate file.

# 🔗 How to Sync a Bitcoin v0.2 — Revelation Edition Node

This guide explains **exactly how a new node syncs automatically over the internet** while other nodes are already mining.

No trusted servers.
No checkpoints.
Proof-of-Work decides truth.

Repository:
[https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2](https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2)

---

## 🧠 What “sync” means in this network

When a new node joins, it will:

1. Create or load the local blockchain
2. Connect to peers over P2P
3. Request missing blocks
4. Download blocks from peers
5. Validate every block locally
6. Catch up to the current height
7. Start mining and broadcasting

Mining **never stops** during sync.

---

## 0️⃣ Requirements

* Linux / macOS / Windows (WSL) / Termux
* Rust (stable)
* Git
* Internet connection
* An open port (default: `8333`)

---

## 1️⃣ Clone the public repository

```bash
git clone https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2.git
cd bitcoin-0.2
```

Cloning the repo does **not** give trust.
Trust comes only from validating Proof-of-Work.

---

## 2️⃣ Build the node

```bash
cargo build --release
```

The binary will be created automatically.

---

## 3️⃣ Start the node

```bash
cargo run --release
```

On first run, the node will:

* Create the `data/` directory
* Load existing blocks if present
* Create the genesis block if needed

You will see:

```text
⛓ Bitcoin v0.2 — Revelation Edition
🌐 P2P listening on 0.0.0.0:8333
🔄 Requesting sync from peers
```

---

## 4️⃣ Peer connection (automatic)

If peers are reachable, the node will:

* Open TCP connections
* Exchange messages
* Remain connected in the background

Peers may be:

* Public internet nodes
* LAN nodes
* VPS nodes
* Home nodes

There is no central server.

---

## 5️⃣ Automatic sync begins

After connecting, the node sends:

```text
SyncRequest { from_height: <local height> }
```

Peers respond by sending **real blocks**, not headers or hashes.

For each received block:

* Proof-of-Work is verified
* Previous hash is checked
* Merkle root is verified
* The block is added to the chain
* UTXO set is rebuilt

You will see output like:

```text
📥 Sync progress | height 152
📥 Sync progress | height 153
```

---

## 6️⃣ Sync completion

Once blocks stop arriving and the height stabilizes:

```text
✅ Sync complete at height XXX
```

The node automatically switches to **normal mode**.

No restart required.

---

## 7️⃣ Normal operation (after sync)

After syncing, the node will:

* Mine new blocks
* Broadcast mined blocks to peers
* Accept blocks mined by others
* Stay in consensus via PoW

Example output:

```text
📊 Blockchain Status:
Height: 1204
Difficulty: 2
UTXO Set Size: 1204
Connected Peers: 5
```

---

## 8️⃣ Restarting a node (fast sync)

If you stop and restart the node:

* Blocks are loaded from `data/blocks.json`
* UTXOs are loaded from `data/utxos.json`
* Sync resumes only if new blocks exist

This makes restarts fast.

---

## 🔐 Important rules (do not break)

* Do not edit `blocks.json`
* Do not skip block validation
* Do not trust GitHub commits for consensus
* Do not assume longest chain = best chain
* Proof-of-Work always wins

---

## 🧩 Current limitations (v0.2)

This version intentionally keeps things simple:

* No headers-first sync yet
* No fork reorganization yet
* No DoS protection yet

These are planned for future versions.

---

## 🧠 Mental model to remember

> GitHub distributes code
> Proof-of-Work decides consensus
> Peers exchange blocks
> Every node verifies everything

That is decentralization.

---

End of document.

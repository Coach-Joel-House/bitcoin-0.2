use serde::{Serialize, Deserialize};

use crate::transaction::Transaction;
use crate::crypto::sha256;

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub timestamp: i64,
    pub prev_hash: Vec<u8>,
    pub nonce: u64,
    pub difficulty: u32,
    pub merkle_root: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: Vec<u8>,
}

impl Block {
    /// Double-SHA256 header hash
    ///
    /// ⚠ CONSENSUS
    /// Must never change once deployed
    pub fn hash_header(&self) -> Vec<u8> {
        sha256(&sha256(
            &bincode::serialize(&self.header)
                .expect("block header serialization"),
        ))
    }

    /// Proof-of-Work verification
    ///
    /// ⚠ CONSENSUS
    /// Single source of truth for PoW validity
    pub fn verify_pow(&self) -> bool {
        self.hash == self.hash_header()
            && crate::pow::valid_pow(
                &self.hash,
                self.header.difficulty,
            )
    }

    /// Deterministic Merkle root computation
    ///
    /// ⚠ CONSENSUS
    pub fn calculate_merkle_root(&self) -> Vec<u8> {
        if self.transactions.is_empty() {
            return vec![0u8; 32];
        }

        let mut hashes: Vec<Vec<u8>> =
            self.transactions
                .iter()
                .map(|tx| tx.txid())
                .collect();

        while hashes.len() > 1 {
            if hashes.len() % 2 != 0 {
                hashes.push(hashes.last().unwrap().clone());
            }

            hashes = hashes
                .chunks(2)
                .map(|pair| {
                    sha256(
                        &[
                            pair[0].as_slice(),
                            pair[1].as_slice(),
                        ]
                        .concat(),
                    )
                })
                .collect();
        }

        hashes[0].clone()
    }
}

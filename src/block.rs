use std::io::Write;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use bincode::{Decode, Encode};
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Debug, Encode, Decode, Deserialize, Clone)]
pub struct Block {
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: i32,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_block_hash: Vec<u8>) -> Block {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            transactions,
            prev_block_hash,
            hash: vec![],
            nonce: 0,
        };

        let proof_of_work = super::proof_of_work::ProofOfWork::new(&block);
        let (nonce, hash) = proof_of_work.run();

        block.hash = hash;
        block.nonce = nonce;

        block
    }

    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        let mut first_transaction = Vec::new();
        first_transaction.push(coinbase);
        Block::new(first_transaction, vec![])
    }

    pub fn hash_transactions(&self) -> Vec<u8>  {

        let mut tx_hashes: Vec<&[u8]> = Vec::new();
        for tx in &self.transactions {
            tx_hashes.push(&tx.id);
        }
        let mut concatenated = Vec::new();
        for hash in tx_hashes {
            concatenated.write_all(hash).unwrap();
        }
        let mut hasher = Sha256::new();
        hasher.update(&concatenated);
        let tx_hash = hasher.finalize();

        tx_hash.to_vec()
    }
}
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode, Deserialize, Clone)]
pub struct Block {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: i32,
}

impl Block {
    pub fn new(data: String, prev_block_hash: Vec<u8>) -> Block {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            data: data.into_bytes(),
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

    pub fn new_genesis_block() -> Block {
        Block::new("Genesis Block".to_string(), vec![])
    }
}
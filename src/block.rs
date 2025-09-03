use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use crate::proof_of_work;
use crate::proof_of_work::ProofOfWork;

#[derive(Debug,Clone)]
pub struct Block {
    pub data: String,
    pub timestamp: i64,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: i32
}

impl Block {
    pub fn god_block() -> Block {
        let mut block = Block {
            data: "the god block".to_string(),
            timestamp: 0,
            prev_block_hash: Vec::new(),
            hash: vec![],
            nonce: 0
        };
        Self::set_hash(&mut block);
        block
    }
    pub fn new(_data: String, _prev_block_hash: Vec<u8>) -> Block {
        let mut block = Block {
            data: _data,
            timestamp: 0,
            prev_block_hash: _prev_block_hash,
            hash: vec![],
            nonce: 0
        };
        let mut block2 = block.clone();
        let pow = ProofOfWork::new(block);
        let (nonce, hash) = pow.run();

        block2.hash = hash;
        block2.nonce = i32::try_from(nonce).unwrap();
        block2
    }
    pub fn set_hash(&mut self) {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("系统时间早于 1970 年");
        let timestamp_secs = duration.as_secs();
        let mut combined = Vec::new();
        combined.extend_from_slice(&self.data.as_bytes());
        combined.extend(&self.prev_block_hash);
        combined.extend(timestamp_secs.to_be_bytes());
        let mut hasher = Sha256::new();
        hasher.update(&combined);
        self.hash = hasher.finalize().to_vec();
    }
}


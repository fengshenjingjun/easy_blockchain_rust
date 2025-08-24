use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
    data: String,
    timestamp: u64,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}


pub struct BlockChain {
    blocks: Vec<Block>,
}

impl Block {
    pub fn god_block() -> Block {
        let mut block = Block {
            data: "the god block".to_string(),
            timestamp: 0,
            prev_block_hash: Vec::new(),
            hash: vec![],
        };
        Self::set_hash(&mut block);
        block
    }
    pub fn new(_data: String, _prev_block_hash: Vec<u8>) -> Block {
        let block = Block {
            data: _data,
            timestamp: 0,
            prev_block_hash: _prev_block_hash,
            hash: vec![],
        };
        // Self::set_hash();
        block
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

impl BlockChain {
    pub fn new() -> BlockChain {
        let god_block = Block::god_block();
        let mut vec = Vec::new();
        vec.push(god_block);
        BlockChain { blocks: vec }

    }
    pub fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.get(self.blocks.len() - 1).unwrap().prev_block_hash.clone();
        let mut new_block = Block {
            data,
            timestamp: 0,
            prev_block_hash: prev_block,
            hash: vec![],
        };
        new_block.set_hash();
        self.blocks.push(new_block);
    }
    // 返回不可变引用的迭代器
    pub fn iter(&self) -> std::slice::Iter<'_, Block> {
        self.blocks.iter()
    }
}

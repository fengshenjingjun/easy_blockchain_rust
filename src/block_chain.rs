use std::fs;
use std::path::Path;
use bincode::config;

const DB_FILE: &str = "blockchain.db";

#[derive(Debug)]
pub struct Blockchain {
    tip: Vec<u8>,
    db: sled::Db,
}

#[derive(Debug)]
pub struct BlockchainIterator {
    current_hash: Vec<u8>,
    db: sled::Db,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        // 创建或打开数据库
        let db = sled::open(DB_FILE).expect("Failed to open database");

        let tip = match db.get("l").expect("Failed to get last block") {
            Some(last_hash) => last_hash.to_vec(),
            None => {
                println!("No existing blockchain found. Creating a new one...");
                let genesis = super::block::Block::new_genesis_block();
                let encoded = bincode::encode_to_vec(&genesis, config::standard()).expect("Failed to serialize genesis block");

                db.insert(&genesis.hash, encoded).expect("Failed to insert genesis block");
                db.insert("l", &*genesis.hash).expect("Failed to insert last block hash");

                genesis.hash.clone()
            }
        };

        Blockchain { tip, db }
    }

    pub fn add_block(&mut self, data: String) {
        let last_hash = self.tip.clone();
        let new_block = super::block::Block::new(data, last_hash);

        let encoded = bincode::encode_to_vec(&new_block, config::standard()).expect("Failed to serialize block");
        self.db.insert(&new_block.hash, encoded).expect("Failed to insert block");
        self.db.insert("l", &*new_block.hash).expect("Failed to update last block hash");

        self.tip = new_block.hash.clone();
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            db: self.db.clone(),
        }
    }
}

impl BlockchainIterator {
    pub fn next(&mut self) -> Option<super::block::Block> {
        if self.current_hash.is_empty() {
            return None;
        }

        match self.db.get(&self.current_hash).expect("Failed to get block") {
            Some(encoded_block) => {
                // let block: super::block::Block = bincode::decode_from_slice(&encoded_block, config::standard()).expect("Failed to deserialize block");
                let block: super::block::Block = bincode::decode_from_slice(&encoded_block, config::standard()).map(|(res, _)| res).unwrap();
                self.current_hash = block.prev_block_hash.clone();
                Some(block)
            }
            None => None,
        }
    }
}
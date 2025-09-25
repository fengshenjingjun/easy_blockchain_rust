use crate::block::Block;
use crate::transaction::{Transaction, TxOutput};
use bincode::config;
use std::collections::HashMap;

const DB_FILE: &str = "blockchain.db";
const GENESIS_COINBASE_DATA: &str = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

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
    pub fn new(address: &str) -> Blockchain {
        if !db_exists() {
            println!("No existing blockchain found. Create one first.");
            std::process::exit(1);
        }

        let db = sled::open(DB_FILE).expect("Failed to open database");

        let tip = db.get("l").expect("Failed to get last block")
            .expect("Blockchain tip not found")
            .to_vec();

        Blockchain { tip, db }
    }

    pub fn create_blockchain(address: &str) -> Blockchain {
        if db_exists() {
            println!("Blockchain already exists.");
            std::process::exit(1);
        }

        let db = sled::open(DB_FILE).expect("Failed to open database");

        let coinbase_tx = crate::transaction::new_coinbase_tx(address, GENESIS_COINBASE_DATA);
        let genesis = Block::new_genesis_block(coinbase_tx);
        let encoded = bincode::encode_to_vec(&genesis, config::standard()).expect("Failed to serialize genesis block");

        let tip = genesis.hash.clone();

        db.insert(&genesis.hash, encoded).expect("Failed to insert genesis block");
        db.insert("l", &*genesis.hash).expect("Failed to insert last block hash");

        Blockchain { tip, db }
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) {
        let last_hash = self.tip.clone();
        let new_block = Block::new(transactions, last_hash);

        let encoded = bincode::encode_to_vec(&new_block, config::standard()).expect("Failed to serialize block");
        self.db.insert(&new_block.hash, encoded).expect("Failed to insert block");
        self.db.insert("l", &*new_block.hash).expect("Failed to update last block hash");

        self.tip = new_block.hash.clone();
    }

    pub fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut unspent_txs = Vec::new();
        let mut spent_txos: HashMap<String, Vec<i32>> = HashMap::new();
        let mut bci = self.iterator();

        while let Some(block) = bci.next() {
            for tx in &block.transactions {
                let tx_id = hex::encode(&tx.id);

                'outputs: for (out_idx, out) in tx.vout.iter().enumerate() {
                    // Was the output spent?
                    if let Some(spent_outs) = spent_txos.get(&tx_id) {
                        for &spent_out in spent_outs {
                            if spent_out == out_idx as i32 {
                                continue 'outputs;
                            }
                        }
                    }

                    if out.can_be_unlocked_with(address) {
                        unspent_txs.push(tx.clone());
                    }
                }

                if !tx.is_coin_base() {
                    for input in &tx.vin {
                        if input.can_unlock_output_with(address) {
                            let in_tx_id = hex::encode(&input.txid);
                            spent_txos.entry(in_tx_id).or_insert_with(Vec::new).push(input.vout);
                        }
                    }
                }
            }

            if block.prev_block_hash.is_empty() {
                break;
            }
        }

        unspent_txs
    }

    pub fn find_utxo(&self, address: &str) -> Vec<TxOutput> {
        let mut utxos = Vec::new();
        let unspent_transactions = self.find_unspent_transactions(address);

        for tx in unspent_transactions {
            for out in tx.vout {
                if out.can_be_unlocked_with(address) {
                    utxos.push(out);
                }
            }
        }

        utxos
    }

    pub fn find_spendable_outputs(&self, address: &str, amount: i32) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let unspent_txs = self.find_unspent_transactions(address);
        let mut accumulated = 0;

        'work: for tx in unspent_txs {
            let tx_id = hex::encode(&tx.id);

            for (out_idx, out) in tx.vout.iter().enumerate() {
                if out.can_be_unlocked_with(address) && accumulated < amount {
                    accumulated += out.value;
                    unspent_outputs.entry(tx_id.clone()).or_insert_with(Vec::new).push(out_idx as i32);

                    if accumulated >= amount {
                        break 'work;
                    }
                }
            }
        }

        (accumulated, unspent_outputs)
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            db: self.db.clone(),
        }
    }
}

impl BlockchainIterator {
    pub fn next(&mut self) -> Option<Block> {
        if self.current_hash.is_empty() {
            return None;
        }

        match self.db.get(&self.current_hash).expect("Failed to get block") {
            Some(encoded_block) => {
                let (block, _): (Block, _) = bincode::decode_from_slice(&encoded_block, config::standard()).expect("decode err");
                self.current_hash = block.prev_block_hash.clone();
                Some(block)
            }
            None => None,
        }
    }
}

fn db_exists() -> bool {
    std::path::Path::new(DB_FILE).exists()
}
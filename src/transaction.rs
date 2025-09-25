use bincode::{config, Decode, Encode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::block_chain::Blockchain;

const SUBSIDY: i32 = 10;
#[derive(Debug, Encode, Decode, Deserialize, Clone)]
pub struct Transaction {
    pub(crate) id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TxOutput>,
}

#[derive(Debug, Encode, Decode, Deserialize, Clone)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout: i32,
    pub script_sig: String,
}

#[derive(Debug, Encode, Decode, Deserialize, Clone)]
pub struct TxInput {
    pub tx_id: Vec<u8>,
    pub v_out: i32,
    pub script_sig: Vec<u8>,
}

#[derive(Debug, Encode, Decode, Deserialize, Clone)]
pub struct TxOutput {
    pub value: i32,
    pub script_pub_key: String,
}

impl Transaction {
    pub fn is_coin_base(&self) -> bool {
        self.vin.len() > 0 && self.vin[0].txid.len() == 0 && self.vin[0].vout == -1
    }

    pub fn set_id(&mut self) {
        // self.encode()
        let encoded = bincode::encode_to_vec(&*self, config::standard()).expect("Serialization failed");
        let hash = Sha256::digest(&encoded);
        self.id = hash.to_vec();
    }
}

impl TXInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

impl TxOutput {
    pub fn can_be_unlocked_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}
pub fn new_coinbase_tx(to: &str, data: &str) -> Transaction {
    if data.is_empty() {
        format!("Reward to {}", to);
    } else {
        data.to_string();
    };
    let tx_in = TXInput {
        txid: Vec::new(),
        vout: -1,
        script_sig: data.parse().unwrap(),
    };
    let tx_out = TxOutput {
        value: SUBSIDY,
        script_pub_key: to.to_string(),
    };
    let mut tx = Transaction {
        id: vec![],
        vin: vec![tx_in],
        vout: vec![tx_out],
    };
    tx.set_id();
    tx
}
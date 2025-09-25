use sha2::{Sha256, Digest};

const TARGET_BITS: u32 = 24;
const MAX_NONCE: i32 = i32::MAX;

pub struct ProofOfWork<'a> {
    block: &'a super::block::Block,
    target: num_bigint::BigUint,
}

impl<'a> ProofOfWork<'a> {
    pub fn new(block: &'a super::block::Block) -> ProofOfWork<'a> {
        let mut target = num_bigint::BigUint::from(1u32);
        target <<= 256 - TARGET_BITS;

        ProofOfWork {
            block,
            target,
        }
    }

    fn prepare_data(&self, nonce: i32) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.block.prev_block_hash);
        data.extend_from_slice(&self.block.hash_transactions());
        data.extend_from_slice(&self.int_to_hex(self.block.timestamp as i64));
        data.extend_from_slice(&self.int_to_hex(TARGET_BITS as i64));
        data.extend_from_slice(&self.int_to_hex(nonce as i64));
        data
    }

    fn int_to_hex(&self, num: i64) -> Vec<u8> {
        num.to_be_bytes().to_vec()
    }

    pub fn run(&self) -> (i32, Vec<u8>) {
        let mut nonce = 0;

        println!("Mining the block containing {} transactions", self.block.transactions.len());
        let hash2=

            loop {
                let data = self.prepare_data(nonce);
                let mut hasher = Sha256::new();
                hasher.update(&data);
                let hash = hasher.finalize().to_vec();
                println!("\r{}", hex::encode(&hash));

                let hash_int = num_bigint::BigUint::from_bytes_be(&hash);

                if hash_int <= self.target {
                    break hash;
                } else {
                    nonce += 1;
                    if nonce >= MAX_NONCE {
                        panic!("Max nonce reached");
                    }
                }
            };

        println!("\n");

        (nonce, hash2)
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize().to_vec();
        let hash_int = num_bigint::BigUint::from_bytes_be(&hash);

        hash_int <= self.target
    }
}
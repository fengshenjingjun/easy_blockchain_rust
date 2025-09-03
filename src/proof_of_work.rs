use std::ops::Shl;
use num_bigint::{BigInt, BigUint};
use crate::block::Block;
use sha2::{Digest, Sha256};

const TARGET_BITS: i32 = 24;
pub struct ProofOfWork {
    block: Block,
    target: BigInt
}

impl ProofOfWork {
    pub fn new(block: Block) -> ProofOfWork {
        let mut pow = ProofOfWork{
            block,
            target: BigInt::from(1),
        };
        let target_temp = BigUint::from(1u32).shl((256 - 24) as usize);
        pow.target = BigInt::from(target_temp);
        pow
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        // 计算双重 SHA-256 哈希 (区块链常见做法)
        let first_hash = Sha256::digest(&data);
        let final_hash = Sha256::digest(&first_hash);
        // 将哈希字节转换为大整数
        let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &final_hash);
        hash_int < self.target
    }

    fn prepare_data(&self, nonce: i32) -> Vec<u8> {
        // 将整数转换为十六进制字符串的辅助函数
        fn int_to_hex(num: i64) -> Vec<u8> {
            format!("{:x}", num).into_bytes()
        }

        // 拼接所有数据部分
        [
            self.block.prev_block_hash.as_slice(),
            self.block.data.clone().into_bytes().as_slice(),
            int_to_hex(self.block.timestamp).as_slice(),
            int_to_hex(TARGET_BITS as i64).as_slice(),
            int_to_hex(nonce as i64).as_slice(),
        ].concat()
    }

    pub fn run(&self) -> (BigInt, Vec<u8>) {
        let mut nonce = 0;
        let mut hash = Default::default();
        while nonce < i32::MAX {
            let data = self.prepare_data(nonce);
            hash = Sha256::digest(&data);
            println!("hash: {:?}", hash);
            let hash_int = BigUint::from_bytes_be(&hash);
            if BigInt::from(hash_int) < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        (BigInt::from(nonce), hash.as_slice().to_vec())
    }
}
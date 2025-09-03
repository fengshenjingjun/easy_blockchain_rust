use std::hash::Hash;
use crate::block::Block;

pub struct BlockChain {
    blocks: Vec<Block>,
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
        let new_block = Block::new(data, prev_block);
        self.blocks.push(new_block);
    }
    // 返回不可变引用的迭代器
    pub fn iter(&self) -> std::slice::Iter<'_, Block> {
        self.blocks.iter()
    }
}

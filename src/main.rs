use blockchain_rust::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new();
    block_chain.add_block("the first block".to_string());
    block_chain.add_block("the second block".to_string());
    for block in block_chain.iter() {
        println!("{:?}", block);
    }

}

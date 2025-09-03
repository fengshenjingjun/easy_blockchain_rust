use blockchain_rust::block_chain::BlockChain;
use blockchain_rust::proof_of_work::ProofOfWork;

fn main() {
    let mut block_chain = BlockChain::new();
    block_chain.add_block("the first block".to_string());
    block_chain.add_block("the second block".to_string());
    for block in block_chain.iter() {
        let block2 = block.clone();
        let pow = ProofOfWork::new(block2);
        println!("pow:{}", &pow.validate());
        println!("{:?}", block);
    }

}

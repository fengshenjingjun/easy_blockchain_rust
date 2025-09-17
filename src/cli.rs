use std::env;

pub struct CLI {
    bc: super::block_chain::Blockchain,
}

impl CLI {
    pub fn new(blockchain: super::block_chain::Blockchain) -> CLI {
        CLI {
            bc: blockchain,
        }
    }

    fn print_usage(&self) {
        println!("Usage:");
        println!("  addblock -data BLOCK_DATA - add a block to the blockchain");
        println!("  printchain - print all the blocks of the blockchain");
    }

    fn validate_args(&self, args: &[String]) {
        if args.len() < 2 {
            self.print_usage();
            std::process::exit(1);
        }
    }

    fn add_block(&mut self, data: String) {
        self.bc.add_block(data);
        println!("Success!");
    }

    fn print_chain(&self) {
        let mut bci = self.bc.iterator();

        while let Some(block) = bci.next() {
            println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
            println!("Data: {}", String::from_utf8_lossy(&block.data));
            println!("Hash: {}", hex::encode(&block.hash));
            
            let proof_of_work = super::proof_of_work::ProofOfWork::new(&block);
            println!("PoW: {}", proof_of_work.validate());
            println!();
        }
    }

    pub fn run(&mut self) {
        let args: Vec<String> = env::args().collect();
        self.validate_args(&args);

        match args[1].as_str() {
            "addblock" => {
                if args.len() < 4 || args[2] != "-data" {
                    self.print_usage();
                    std::process::exit(1);
                }
                let data = args[3].clone();
                self.add_block(data);
            }
            "printchain" => {
                self.print_chain();
            }
            _ => {
                self.print_usage();
                std::process::exit(1);
            }
        }
    }
}
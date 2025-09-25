use std::env;
use crate::block_chain::Blockchain;
use crate::transaction::{new_coinbase_tx, Transaction, TXInput, TxOutput};

pub struct CLI;

impl CLI {
    pub fn new() -> CLI {
        CLI
    }

    fn create_blockchain(&self, address: &str) {
        let bc = Blockchain::create_blockchain(address);
        println!("Done!");
    }

    fn get_balance(&self, address: &str) {
        let bc = Blockchain::new(address);
        let utxos = bc.find_utxo(address);

        let balance = utxos.iter().map(|out| out.value).sum::<i32>();
        println!("Balance of '{}': {}", address, balance);
    }

    fn print_usage(&self) {
        println!("Usage:");
        println!("  getbalance -address ADDRESS - Get balance of ADDRESS");
        println!("  createblockchain -address ADDRESS - Create a blockchain and send genesis block reward to ADDRESS");
        println!("  printchain - Print all the blocks of the blockchain");
        println!("  send -from FROM -to TO -amount AMOUNT - Send AMOUNT of coins from FROM address to TO");
    }

    fn validate_args(&self, args: &[String]) {
        if args.len() < 2 {
            self.print_usage();
            std::process::exit(1);
        }
    }

    fn print_chain(&self) {
        // TODO: Fix this
        let bc = Blockchain::new("");
        let mut bci = bc.iterator();

        while let Some(block) = bci.next() {
            println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
            println!("Hash: {}", hex::encode(&block.hash));

            let proof_of_work = super::proof_of_work::ProofOfWork::new(&block);
            println!("PoW: {}", proof_of_work.validate());
            println!();
        }
    }

    fn send(&self, from: &str, to: &str, amount: i32) {
        let mut bc = Blockchain::new(from);
        let tx = self.new_utxo_transaction(from, to, amount, &bc);
        bc.mine_block(vec![tx]);
        println!("Success!");
    }

    fn new_utxo_transaction(&self, from: &str, to: &str, amount: i32, bc: &Blockchain) -> Transaction {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        let (acc, valid_outputs) = bc.find_spendable_outputs(from, amount);

        if acc < amount {
            panic!("ERROR: Not enough funds");
        }

        // Build a list of inputs
        for (txid, outs) in valid_outputs {
            let tx_id = hex::decode(&txid).expect("Failed to decode txid");

            for out in outs {
                let input = TXInput {
                    txid: tx_id.clone(),
                    vout: out,
                    script_sig: from.to_string(),
                };
                inputs.push(input);
            }
        }

        // Build a list of outputs
        outputs.push(TxOutput {
            value: amount,
            script_pub_key: to.to_string(),
        });

        if acc > amount {
            outputs.push(TxOutput {
                value: acc - amount,
                script_pub_key: from.to_string(),
            }); // a change
        }

        let mut tx = Transaction {
            id: vec![],
            vin: inputs,
            vout: outputs,
        };
        tx.set_id();

        tx
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.validate_args(&args);

        match args[1].as_str() {
            "getbalance" => {
                let address = args.get(3).expect("Address is required");
                self.get_balance(address);
            }
            "createblockchain" => {
                let address = args.get(3).expect("Address is required");
                self.create_blockchain(address);
            }
            "printchain" => {
                self.print_chain();
            }
            "send" => {
                let from = args.get(3).expect("From address is required");
                let to = args.get(5).expect("To address is required");
                let amount: i32 = args.get(7).expect("Amount is required").parse().expect("Invalid amount");
                self.send(from, to, amount);
            }
            _ => {
                self.print_usage();
                std::process::exit(1);
            }
        }
    }
}
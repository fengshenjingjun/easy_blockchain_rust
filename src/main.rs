mod block;
mod block_chain;
mod proof_of_work;
mod cli;
mod transaction;
fn main() {
    let mut cli = cli::CLI {};
    cli.run();

}

mod block;
mod block_chain;
mod proof_of_work;
mod cli;
fn main() {
    let bc = block_chain::Blockchain::new();
    let mut cli = cli::CLI::new(bc);
    cli.run();

}

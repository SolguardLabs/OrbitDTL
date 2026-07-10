mod accounts;
mod amount;
mod asset;
mod cli;
mod codec;
mod error;
mod events;
mod ids;
mod ledger;
mod oracle;
mod orders;
mod risk;
mod routes;
mod session;
mod vault;

use clap::Parser;

fn main() {
    if let Err(error) = cli::Cli::parse().execute() {
        eprintln!("orbit-dtl: {error}");
        std::process::exit(1);
    }
}

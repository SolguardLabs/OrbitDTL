use crate::codec::to_pretty_json;
use crate::error::OrbitResult;
use crate::ledger::Ledger;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "orbit-dtl")]
#[command(about = "Orbit DTL settlement console")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Demo {
        #[arg(long)]
        json: bool,
    },
}

impl Cli {
    pub fn execute(self) -> OrbitResult<()> {
        match self.command {
            Command::Demo { json } => {
                let report = Ledger::demo()?;
                if json {
                    println!("{}", to_pretty_json(&report)?);
                } else {
                    let tracked_balances = report
                        .accounts
                        .values()
                        .map(|account| account.balances().len())
                        .sum::<usize>();
                    let reserve_units = report
                        .vaults
                        .values()
                        .map(|vault| vault.reserve.raw())
                        .sum::<u128>();
                    println!("Orbit DTL demo executed");
                    println!("accounts: {}", report.accounts.len());
                    println!("tracked_balances: {tracked_balances}");
                    println!("vaults: {}", report.vaults.len());
                    println!("reserve_units: {reserve_units}");
                    println!("sessions: {}", report.sessions.len());
                    println!("events: {}", report.events.len());
                }
            }
        }

        Ok(())
    }
}

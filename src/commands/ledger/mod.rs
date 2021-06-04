use crate::lib::error::NnsCliResult;

use clap::Clap;
use ic_agent::Agent;
use ic_types::Principal;

mod account_id;
mod balance;

/// Call the ledger canister
#[derive(Clap)]
#[clap(name("ledger"))]
pub struct LedgerOpts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    AccountId(account_id::AccountIdOpts),
    Balance(balance::BalanceOpts),
}

pub async fn exec(opts: LedgerOpts, agent: Agent, sender: Principal) -> NnsCliResult {
    match opts.subcmd {
        SubCommand::AccountId(v) => account_id::exec(v, sender).await,
        SubCommand::Balance(v) => balance::exec(v, agent, sender).await,
    }
}

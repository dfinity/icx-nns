use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;

use clap::Clap;

mod governance;
mod ledger;
mod neuron;
mod principal;

#[derive(Clap)]
pub enum Command {
    GetPrincipal(principal::GetPrincipalOpts),
    Governance(governance::GovernanceOpts),
    Ledger(ledger::LedgerOpts),
    Neuron(neuron::NeuronOpts),
}

pub async fn exec(cmd: Command, env: Env) -> NnsCliResult {
    match cmd {
        Command::GetPrincipal(v) => principal::exec(v, env).await,
        Command::Governance(v) => governance::exec(v, env).await,
        Command::Ledger(v) => ledger::exec(v, env).await,
        Command::Neuron(v) => neuron::exec(v, env).await,
    }
}

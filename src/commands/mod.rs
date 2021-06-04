use crate::lib::agent::construct_agent;
use crate::lib::error::NnsCliResult;

use clap::Clap;
use tokio::runtime::Runtime;

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

pub fn exec(cmd: Command, endpoint: Option<String>, use_hsm: bool) -> NnsCliResult {
    let runtime = Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(async {
        let (agent, sender) = construct_agent(endpoint, use_hsm).await?;
        match cmd {
            Command::GetPrincipal(v) => principal::exec(v, sender).await,
            Command::Governance(v) => governance::exec(v, agent).await,
            Command::Ledger(v) => ledger::exec(v, agent, sender).await,
            Command::Neuron(v) => neuron::exec(v, agent).await,
        }
    })
}

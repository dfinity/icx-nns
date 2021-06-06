use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::segregated_sign_send::sign::SignPayload;

use clap::Clap;

mod governance;
mod ledger;
mod neuron;
mod principal;
mod send;

#[derive(Clap)]
pub enum Command {
    GetPrincipal(principal::GetPrincipalOpts),
    Governance(governance::GovernanceOpts),
    Ledger(ledger::LedgerOpts),
    Send(send::SendOpts),
    Neuron(neuron::NeuronOpts),
}

pub async fn exec(cmd: Command, maybe_sign_payload: Option<SignPayload>, env: Env) -> NnsCliResult {
    match cmd {
        Command::GetPrincipal(v) => principal::exec(v, env).await,
        Command::Governance(v) => governance::exec(v, maybe_sign_payload, env).await,
        Command::Ledger(v) => ledger::exec(v, maybe_sign_payload, env).await,
        Command::Send(v) => send::exec(v).await,
        Command::Neuron(v) => neuron::exec(v, maybe_sign_payload, env).await,
    }
}

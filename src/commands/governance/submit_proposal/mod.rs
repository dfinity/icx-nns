use crate::lib::error::NnsCliResult;

use clap::Clap;
use ic_agent::Agent;

mod authorize_to_subnet;
mod motion;

/// Submit a proposal
#[derive(Clap, Clone)]
#[clap(name("governance"))]
pub struct SubmitProposalOpts {
    #[clap(subcommand)]
    subcmd: SubCommand,

    /// Neuron id
    neuron_id: u64,

    /// Summary
    #[clap(long)]
    summary: String,

    /// Url
    #[clap(long)]
    url: String,
}

#[derive(Clap, Clone)]
enum SubCommand {
    AuthorizeToSubnet(authorize_to_subnet::AuthToSubnetOpts),
    Motion(motion::MotionOpts),
}

pub async fn exec(opts: SubmitProposalOpts, agent: Agent) -> NnsCliResult {
    let proposal_opts = opts.clone();
    match opts.subcmd {
        SubCommand::AuthorizeToSubnet(v) => {
            authorize_to_subnet::exec(v, proposal_opts, agent.clone()).await
        }
        SubCommand::Motion(v) => motion::exec(v, proposal_opts, agent.clone()).await,
    }
}

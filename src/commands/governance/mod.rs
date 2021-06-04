use crate::lib::error::NnsCliResult;

use clap::Clap;
use ic_agent::Agent;

mod get_pending_proposals;
mod get_proposal_info;
mod list_proposals;
mod submit_proposal;

/// Call the governance canister
#[derive(Clap)]
#[clap(name("governance"))]
pub struct GovernanceOpts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    GetPendingProposals(get_pending_proposals::GetPendingProposalsOpts),
    GetProposalInfo(get_proposal_info::GetProposalInfoOpts),
    ListProposals(list_proposals::ListPropsalOpts),
    SubmitProposal(submit_proposal::SubmitProposalOpts),
}

pub async fn exec(opts: GovernanceOpts, agent: Agent) -> NnsCliResult {
    match opts.subcmd {
        SubCommand::GetPendingProposals(v) => get_pending_proposals::exec(v, agent.clone()).await,
        SubCommand::GetProposalInfo(v) => get_proposal_info::exec(v, agent.clone()).await,
        SubCommand::ListProposals(v) => list_proposals::exec(v, agent.clone()).await,
        SubCommand::SubmitProposal(v) => submit_proposal::exec(v, agent.clone()).await,
    }
}

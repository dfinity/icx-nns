use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;

use clap::Clap;

mod get_pending_proposals;
mod get_proposal_info;
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
    SubmitProposal(submit_proposal::SubmitProposalOpts),
}

pub async fn exec(opts: GovernanceOpts, env: Env) -> NnsCliResult {
    match opts.subcmd {
        SubCommand::GetPendingProposals(v) => get_pending_proposals::exec(v, env).await,
        SubCommand::GetProposalInfo(v) => get_proposal_info::exec(v, env).await,
        SubCommand::SubmitProposal(v) => submit_proposal::exec(v, env).await,
    }
}

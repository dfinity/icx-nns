use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::segregated_sign_send::sign::SignPayload;

use clap::Clap;

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

pub async fn exec(
    opts: GovernanceOpts,
    maybe_sign_payload: Option<SignPayload>,
    env: Env,
) -> NnsCliResult {
    match opts.subcmd {
        SubCommand::GetPendingProposals(v) => get_pending_proposals::exec(v, env).await,
        SubCommand::GetProposalInfo(v) => get_proposal_info::exec(v, env).await,
        SubCommand::ListProposals(v) => list_proposals::exec(v, env).await,
        SubCommand::SubmitProposal(v) => submit_proposal::exec(v, maybe_sign_payload, env).await,
    }
}

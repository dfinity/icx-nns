use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;

use clap::Clap;

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
    GetProposalInfo(get_proposal_info::GetProposalInfoOpts),
    SubmitProposal(submit_proposal::SubmitProposalOpts),
}

pub async fn exec(opts: GovernanceOpts, env: Env) -> NnsCliResult {
    match opts.subcmd {
        SubCommand::GetProposalInfo(v) => get_proposal_info::exec(v, env).await,
        SubCommand::SubmitProposal(v) => submit_proposal::exec(v, env).await,
    }
}

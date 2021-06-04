use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{Decode, Encode};
use clap::Clap;
use ic_nns_governance::pb::v1::ProposalInfo;

const GET_PROPOSAL_INFO_METHOD: &str = "get_proposal_info";

/// Call governance canister's get_proposal_info method
#[derive(Clap)]
pub struct GetProposalInfoOpts {
    /// Proposal id
    id: u64,
}

pub async fn exec(opts: GetProposalInfoOpts, env: Env) -> NnsCliResult {
    let result = env
        .agent
        .query(&governance_canister_id(), GET_PROPOSAL_INFO_METHOD)
        .with_arg(Encode!(&(opts.id))?)
        .call()
        .await?;

    let maybe_proposal = Decode!(&result, Option<ProposalInfo>)?;

    match maybe_proposal {
        Some(proposal) => println!("{:?}", proposal),
        None => println!("No proposal found with id {}", opts.id),
    };

    NnsCliResult::Ok(())
}

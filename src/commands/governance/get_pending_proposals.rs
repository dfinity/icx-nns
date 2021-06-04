use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{Decode, Encode};
use clap::Clap;
use ic_nns_governance::pb::v1::ProposalInfo;

const GET_PENDING_PROPOSALS_METHOD: &str = "get_pending_proposals";

/// Call governance canister's get_pending_proposals method
#[derive(Clap)]
pub struct GetPendingProposalsOpts {}

pub async fn exec(_opts: GetPendingProposalsOpts, env: Env) -> NnsCliResult {
    let result = env
        .agent
        .query(&governance_canister_id(), GET_PENDING_PROPOSALS_METHOD)
        .with_arg(Encode!(&())?)
        .call()
        .await?;

    let proposals = Decode!(&result, Vec<ProposalInfo>)?;

    println!("{:?}", proposals);

    NnsCliResult::Ok(())
}

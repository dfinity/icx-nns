use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::{
    governance_canister_id, ProposalRewardStatus, ProposalStatus, Topic,
};
use ic_nns_governance::pb::v1::{ListProposalInfo, ListProposalInfoResponse};

use candid::{CandidType, Decode, Encode};
use clap::Clap;
use num_traits::ToPrimitive;
use std::str::FromStr;

const LIST_PROPOSALS_METHOD: &str = "list_proposals";

/// Call governance canister's list_proposals method
#[derive(CandidType, Clap)]
pub struct ListPropsalOpts {
    #[clap(long,
        possible_values = &["unspecified",
        "accept_votes", "ready_to_settle",
        "settled", "ineligible"])]
    include_reward_status: Option<Vec<String>>,

    #[clap(long)]
    before_proposal: Option<u64>,

    #[clap(long, default_value = "100")]
    limit: u32,

    #[clap(long,
        possible_values = &["unspecified",
        "neuron_management", "exchange_rate", "network_economics",
        "governance", "node_admin", "participant_management",
        "subnet_management", "network_canister_management",
        "kyc", "node_provider_rewards"])]
    exclude_topic: Option<Vec<String>>,

    #[clap(long,
        possible_values = &["unspecified",
        "open", "rejected", "adopted",
        "executed", "failed"])]
    include_status: Option<Vec<String>>,
}

pub async fn exec(opts: ListPropsalOpts, env: Env) -> NnsCliResult {
    let include_reward_status: Vec<i32> = match opts.include_reward_status {
        Some(vec) => vec
            .iter()
            .map(|v| ToPrimitive::to_i32(&ProposalRewardStatus::from_str(&v).unwrap()).unwrap())
            .collect(),
        None => Vec::<i32>::new(),
    };

    let exclude_topic: Vec<i32> = match opts.exclude_topic {
        Some(vec) => vec
            .iter()
            .map(|v| ToPrimitive::to_i32(&Topic::from_str(&v).unwrap()).unwrap())
            .collect(),
        None => Vec::<i32>::new(),
    };

    let include_status: Vec<i32> = match opts.include_status {
        Some(vec) => vec
            .iter()
            .map(|v| ToPrimitive::to_i32(&ProposalStatus::from_str(&v).unwrap()).unwrap())
            .collect(),
        None => Vec::<i32>::new(),
    };

    let before_proposal = opts
        .before_proposal
        .map(|v| ic_nns_common::pb::v1::ProposalId { id: v });
    let limit = opts.limit;

    let proposal_info = ListProposalInfo {
        include_status,
        include_reward_status,
        exclude_topic,
        before_proposal,
        limit,
    };

    let result = env
        .agent
        .query(&governance_canister_id(), LIST_PROPOSALS_METHOD)
        .with_arg(Encode!(&proposal_info)?)
        .call()
        .await?;

    let proposal_info_response = Decode!(&result, ListProposalInfoResponse)?;
    println!("{:?}", proposal_info_response);

    NnsCliResult::Ok(())
}

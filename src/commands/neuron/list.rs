use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{CandidType, Decode, Encode};
use clap::Clap;
use ic_agent::Agent;
use ic_nns_governance::pb::v1::ListNeuronsResponse;

const LIST_NEURONS_METHOD: &str = "list_neurons";

/// Call governance canister's list_neurons method
#[derive(CandidType, Clap)]
pub struct ListNeuronsOpts {
    // List of neuron ids
    #[clap(long)]
    neuron_ids: Vec<u64>,

    // Include neurons readable by caller
    #[clap(long)]
    include_neurons_readable_by_caller: bool,
}

pub async fn exec(opts: ListNeuronsOpts, agent: Agent) -> NnsCliResult {
    let result = agent
        .query(&governance_canister_id(), LIST_NEURONS_METHOD)
        .with_arg(Encode!(&opts)?)
        .call()
        .await?;

    let neurons = Decode!(&result, ListNeuronsResponse)?;

    println!("{:?}", neurons);
    NnsCliResult::Ok(())
}

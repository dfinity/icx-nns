use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{Decode, Encode};
use clap::Clap;

const GET_NEURON_IDS_METHOD: &str = "get_neuron_ids";

/// Get the neuron ids associated with your identity
#[derive(Clap)]
pub struct GetNeuronIdOpts {}

pub async fn exec(_opts: GetNeuronIdOpts, env: Env) -> NnsCliResult {
    let arg = Encode!(&())?;

    let result = env
        .agent
        .query(&governance_canister_id(), GET_NEURON_IDS_METHOD)
        .with_arg(arg)
        .call()
        .await?;

    let ids = Decode!(&result, Vec<u64>)?;

    println!("{:?}", ids);

    NnsCliResult::Ok(())
}

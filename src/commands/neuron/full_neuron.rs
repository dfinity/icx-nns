use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{Decode, Encode};
use clap::Clap;
use ic_nns_governance::pb::v1::{GovernanceError, Neuron};

const GET_FULL_NEURON_METHOD: &str = "get_full_neuron";

/// Get the full neuron information
#[derive(Clap)]
pub struct GetFullNeuronOpts {}

pub async fn exec(_opts: GetFullNeuronOpts, id: u64, env: Env) -> NnsCliResult {
    let arg = Encode!(&id)?;

    let result = env
        .agent
        .query(&governance_canister_id(), GET_FULL_NEURON_METHOD)
        .with_arg(arg)
        .call()
        .await?;

    let neuron_result = Decode!(&result, Result<Neuron, GovernanceError>)?;

    match neuron_result {
        Ok(neuron) => println!("{:?}", neuron),
        Err(gov_err) => eprintln!("{:?}", gov_err),
    };

    NnsCliResult::Ok(())
}

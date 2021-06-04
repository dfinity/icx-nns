use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{Decode, Encode};
use clap::Clap;
use ic_agent::Agent;
use ic_nns_governance::pb::v1::{GovernanceError, Neuron};

const GET_FULL_NEURON_METHOD: &str = "get_full_neuron";

/// Get the full neuron information
#[derive(Clap)]
pub struct GetFullNeuronOpts {}

pub async fn exec(_opts: GetFullNeuronOpts, id: u64, agent: Agent) -> NnsCliResult {
    let result = agent
        .query(&governance_canister_id(), GET_FULL_NEURON_METHOD)
        .with_arg(Encode!(&id)?)
        .call()
        .await?;

    let neuron_result = Decode!(&result, Result<Neuron, GovernanceError>)?;

    match neuron_result {
        Ok(neuron) => println!("{:?}", neuron),
        Err(gov_err) => eprintln!("{:?}", gov_err),
    };

    NnsCliResult::Ok(())
}

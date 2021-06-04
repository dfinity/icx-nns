use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{Decode, Encode};
use clap::Clap;
use ic_nns_governance::pb::v1::{GovernanceError, NeuronInfo};

const GET_NEURON_INFO_METHOD: &str = "get_neuron_info";

/// Get some neuron info
#[derive(Clap)]
pub struct GetNeuronInfoOpts {}

pub async fn exec(_opts: GetNeuronInfoOpts, id: u64, env: Env) -> NnsCliResult {
    let arg = Encode!(&id)?;

    let result = env
        .agent
        .query(&governance_canister_id(), GET_NEURON_INFO_METHOD)
        .with_arg(arg)
        .call()
        .await?;

    let neuron_info_result = Decode!(&result, Result<NeuronInfo, GovernanceError>)?;

    match neuron_info_result {
        Ok(neuron_info) => println!("{:?}", neuron_info),
        Err(gov_err) => eprintln!("{:?}", gov_err),
    };

    NnsCliResult::Ok(())
}

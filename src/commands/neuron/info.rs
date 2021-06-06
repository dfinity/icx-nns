use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;
use crate::lib::segregated_sign_send::sign::{sign_message, CanisterPayload, SignPayload};

use candid::{Decode, Encode};
use clap::Clap;
use ic_nns_governance::pb::v1::{GovernanceError, NeuronInfo};

const GET_NEURON_INFO_METHOD: &str = "get_neuron_info";

/// Get some neuron info
#[derive(Clap)]
pub struct GetNeuronInfoOpts {}

pub async fn exec(
    _opts: GetNeuronInfoOpts,
    id: u64,
    maybe_sign_payload: Option<SignPayload>,
    env: Env,
) -> NnsCliResult {
    let arg = Encode!(&id)?;

    match maybe_sign_payload {
        Some(payload) => {
            let mut sign_payload = payload;
            sign_payload.payload = Some(CanisterPayload {
                canister_id: governance_canister_id(),
                method_name: GET_NEURON_INFO_METHOD.to_string(),
                is_query: true,
                arg,
            });
            sign_message(sign_payload, env.agent, env.sender).await?;
        }
        None => {
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
        }
    }

    NnsCliResult::Ok(())
}

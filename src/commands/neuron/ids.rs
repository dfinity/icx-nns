use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;
use crate::lib::segregated_sign_send::sign::{sign_message, CanisterPayload, SignPayload};

use candid::{Decode, Encode};
use clap::Clap;

const GET_NEURON_IDS_METHOD: &str = "get_neuron_ids";

/// Get the neuron ids associated with your identity
#[derive(Clap)]
pub struct GetNeuronIdOpts {}

pub async fn exec(
    _opts: GetNeuronIdOpts,
    maybe_sign_payload: Option<SignPayload>,
    env: Env,
) -> NnsCliResult {
    let arg = Encode!(&())?;
    match maybe_sign_payload {
        Some(payload) => {
            let mut sign_payload = payload;
            sign_payload.payload = Some(CanisterPayload {
                canister_id: governance_canister_id(),
                method_name: GET_NEURON_IDS_METHOD.to_string(),
                is_query: true,
                arg,
            });
            sign_message(sign_payload, env.agent, env.sender).await?;
        }
        None => {
            let result = env
                .agent
                .query(&governance_canister_id(), GET_NEURON_IDS_METHOD)
                .with_arg(arg)
                .call()
                .await?;

            let ids = Decode!(&result, Vec<u64>)?;

            println!("{:?}", ids);
        }
    }

    NnsCliResult::Ok(())
}

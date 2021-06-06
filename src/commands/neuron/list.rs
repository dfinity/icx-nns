use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;
use crate::lib::segregated_sign_send::sign::{sign_message, CanisterPayload, SignPayload};

use candid::{CandidType, Decode, Encode};
use clap::Clap;
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

pub async fn exec(
    opts: ListNeuronsOpts,
    maybe_sign_payload: Option<SignPayload>,
    env: Env,
) -> NnsCliResult {
    let arg = Encode!(&opts)?;

    match maybe_sign_payload {
        Some(payload) => {
            let mut sign_payload = payload;
            sign_payload.payload = Some(CanisterPayload {
                canister_id: governance_canister_id(),
                method_name: LIST_NEURONS_METHOD.to_string(),
                is_query: true,
                arg,
            });
            sign_message(sign_payload, env.agent, env.sender).await?;
        }
        None => {
            let result = env
                .agent
                .query(&governance_canister_id(), LIST_NEURONS_METHOD)
                .with_arg(arg)
                .call()
                .await?;

            let neurons = Decode!(&result, ListNeuronsResponse)?;

            println!("{:?}", neurons);
        }
    }

    NnsCliResult::Ok(())
}

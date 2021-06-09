use crate::lib::agent::create_waiter;
use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use anyhow::anyhow;
use candid::{Decode, Encode};
use clap::Clap;
use ic_base_types::PrincipalId;
use ic_nns_common::pb::v1::NeuronId as NeuronIdProto;
use ic_nns_common::types::NeuronId;
use ic_nns_governance::pb::v1::manage_neuron_response::Command::Error;
use ic_nns_governance::pb::v1::{
    manage_neuron::configure::Operation, manage_neuron::AddHotKey, manage_neuron::Command,
    manage_neuron::Configure, manage_neuron::RemoveHotKey, ManageNeuron, ManageNeuronResponse,
};
use ic_types::Principal;
use std::convert::TryFrom;

const MANAGE_NEURON_METHOD: &str = "manage_neuron";

/// Configure the neuron's hot key parameters
#[derive(Clap, Clone)]
pub struct HotKeyOpts {
    hot_key: Principal,
    #[clap(possible_values = &["add", "remove"])]
    operation: String,
}

fn get_command(operation: String, hot_key: Principal) -> NnsCliResult<Command> {
    match operation.as_str() {
        "add" => {
            let hot_key = AddHotKey {
                new_hot_key: {
                    let base_types_principal =
                        PrincipalId::try_from(hot_key.as_slice()).map_err(|err| anyhow!(err))?;
                    Some(base_types_principal)
                },
            };
            Ok(Command::Configure(Configure {
                operation: Some(Operation::AddHotKey(hot_key)),
            }))
        }
        "remove" => {
            let hot_key = RemoveHotKey {
                hot_key_to_remove: {
                    let base_types_principal =
                        PrincipalId::try_from(hot_key.as_slice()).map_err(|err| anyhow!(err))?;
                    Some(base_types_principal)
                },
            };
            Ok(Command::Configure(Configure {
                operation: Some(Operation::RemoveHotKey(hot_key)),
            }))
        }
        _ => unreachable!(),
    }
}

pub async fn exec(opts: HotKeyOpts, id: u64, env: Env) -> NnsCliResult {
    let command = get_command(opts.operation, opts.hot_key)?;
    let id = NeuronId(id);

    let manage_neuron = ManageNeuron {
        id: Some(NeuronIdProto::from(id)),
        command: Some(command),
    };
    let arg = Encode!(&manage_neuron)?;

    let result = env
        .agent
        .update(&governance_canister_id(), MANAGE_NEURON_METHOD)
        .with_arg(arg)
        .call_and_wait(create_waiter())
        .await?;
    let manage_neuron_response = Decode!(&result, ManageNeuronResponse)?;

    match manage_neuron_response.command {
        Some(Error(gov_err)) => println!("{}", gov_err),
        Some(ic_nns_governance::pb::v1::manage_neuron_response::Command::Configure(
            ic_nns_governance::pb::v1::manage_neuron_response::ConfigureResponse {},
        )) => eprintln!("Configured succesfully."),
        _ => eprintln!("Received an invalid response."),
    };

    NnsCliResult::Ok(())
}

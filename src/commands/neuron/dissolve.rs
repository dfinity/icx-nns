use crate::lib::agent::create_waiter;
use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use anyhow::anyhow;
use candid::{Decode, Encode};
use clap::Clap;
use ic_nns_common::pb::v1::NeuronId as NeuronIdProto;
use ic_nns_common::types::NeuronId;
use ic_nns_governance::pb::v1::manage_neuron_response::Command::Error;
use ic_nns_governance::pb::v1::{
    manage_neuron::configure::Operation,
    manage_neuron::Command,
    manage_neuron::Configure,
    manage_neuron::IncreaseDissolveDelay,
    manage_neuron::{StartDissolving, StopDissolving},
    ManageNeuron, ManageNeuronResponse,
};

const MANAGE_NEURON_METHOD: &str = "manage_neuron";

/// Configure the neuron's dissolve parameters
#[derive(Clap, Clone)]
pub struct DissolveOpts {
    #[clap(possible_values = &["increase-delay", "start", "stop"])]
    operation: String,
    additional_delay_seconds: Option<u32>,
}

fn get_command(opts: DissolveOpts) -> NnsCliResult<Command> {
    match opts.operation.as_str() {
        "start" => Ok(Command::Configure(Configure {
            operation: Some(Operation::StartDissolving(StartDissolving {})),
        })),
        "stop" => Ok(Command::Configure(Configure {
            operation: Some(Operation::StopDissolving(StopDissolving {})),
        })),
        "increase-delay" => {
            let dissolve_delay = IncreaseDissolveDelay {
                additional_dissolve_delay_seconds:
                    opts.additional_delay_seconds
                        .ok_or_else(
                            || anyhow!(
                                "Please specify a dissolve dissolve delay i.e. `icx-nns neuron <id> dissolve increase-delay <additional-delay-seconds>"))?,
            };
            Ok(Command::Configure(Configure {
                operation: Some(Operation::IncreaseDissolveDelay(dissolve_delay)),
            }))
        }
        _ => unreachable!(),
    }
}

pub async fn exec(opts: DissolveOpts, id: u64, env: Env) -> NnsCliResult {
    let command = get_command(opts)?;
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

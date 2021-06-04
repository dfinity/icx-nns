use crate::lib::agent::create_waiter;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::governance_canister_id;

use candid::{CandidType, Decode, Encode};
use clap::Clap;
use ic_agent::Agent;
use ic_nns_common::pb::v1::NeuronId as NeuronIdProto;
use ic_nns_common::types::NeuronId;
use ic_nns_governance::pb::v1::manage_neuron_response::Command::{Error, MakeProposal};
use ic_nns_governance::pb::v1::proposal::Action;
use ic_nns_governance::pb::v1::NnsFunction::SetAuthorizedSubnetworks;
use ic_nns_governance::pb::v1::{
    manage_neuron::Command, ExecuteNnsFunction, ManageNeuron, ManageNeuronResponse, Proposal,
};

use ic_types::Principal;

const MANAGE_NEURON_METHOD: &str = "manage_neuron";

/// Submit a proposal to authorize a principal to one or more verified application subnetworks
#[derive(Clap, Clone)]
pub struct AuthToSubnetOpts {
    /// The principal to be authorized to create canisters using ICPTs.
    /// If who is `None`, then the proposal will set the default list of subnets
    /// onto which everyone is authorized to create canisters to `subnets`
    /// (except those who have a custom list).
    #[clap(long)]
    who: Option<Principal>,

    /// The list of subnets that `who` would be authorized to create subnets on.
    /// If `subnets` is `None`, then `who` is removed from the list of
    /// authorized users.
    #[clap(long)]
    subnets: Option<Vec<Principal>>,
}

pub async fn exec(
    opts: AuthToSubnetOpts,
    proposal_opts: super::SubmitProposalOpts,
    agent: Agent,
) -> NnsCliResult {
    #[derive(CandidType)]
    struct SetAuthorizedSubnetworkListArgs {
        who: Option<Principal>,
        subnets: Vec<Principal>,
    }
    let payload = Encode!(&SetAuthorizedSubnetworkListArgs {
        who: opts.who,
        subnets: opts.subnets.unwrap_or_default()
    })?;
    let nns_function = SetAuthorizedSubnetworks as i32;
    let execute_nns_function = ExecuteNnsFunction {
        nns_function,
        payload,
    };
    let proposal = Proposal {
        summary: proposal_opts.summary,
        url: proposal_opts.url,
        action: Some(Action::ExecuteNnsFunction(execute_nns_function)),
    };
    let id = NeuronId(proposal_opts.neuron_id);
    let manage_neuron = ManageNeuron {
        id: Some(NeuronIdProto::from(id)),
        command: Some(Command::MakeProposal(Box::<Proposal>::new(proposal))),
    };

    let result = agent
        .update(&governance_canister_id(), MANAGE_NEURON_METHOD)
        .with_arg(Encode!(&manage_neuron)?)
        .call_and_wait(create_waiter())
        .await?;
    let manage_neuron_response = Decode!(&result, ManageNeuronResponse)?;

    match manage_neuron_response.command {
        Some(Error(gov_err)) => println!("{}", gov_err),
        Some(MakeProposal(response)) => match response.proposal_id {
            Some(proposal_id) => println!("{:?}", proposal_id),
            None => eprintln!("Propsal sent but did not receive a proposal id in response."),
        },
        _ => eprintln!("Received an invalid response."),
    };

    NnsCliResult::Ok(())
}

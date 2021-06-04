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
use ic_nns_governance::pb::v1::{
    manage_neuron::Command, ManageNeuron, ManageNeuronResponse, Motion, Proposal,
};

const MANAGE_NEURON_METHOD: &str = "manage_neuron";

/// Submit a motion to the IC.
#[derive(CandidType, Clap, Clone)]
pub struct MotionOpts {
    /// The text of the motion.
    motion_text: String,
}

pub async fn exec(
    opts: MotionOpts,
    proposal_opts: super::SubmitProposalOpts,
    agent: Agent,
) -> NnsCliResult {
    let proposal = Proposal {
        summary: proposal_opts.summary,
        url: proposal_opts.url,
        action: Some(Action::Motion(Motion {
            motion_text: opts.motion_text,
        })),
    };
    let manage_neuron = ManageNeuron {
        id: Some(NeuronIdProto::from(NeuronId(proposal_opts.neuron_id))),
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

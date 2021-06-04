use ic_types::Principal;
use num_derive::ToPrimitive;
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, Debug, EnumString, ToPrimitive)]
#[strum(serialize_all = "snake_case")]
pub enum ProposalRewardStatus {
    Unspecified = 0,
    /// The proposal still accept votes, for the purpose of
    /// vote rewards. This implies nothing on the ProposalStatus.
    AcceptVotes = 1,
    /// The proposal no longer accepts votes. It is due to settle
    /// at the next reward event.
    ReadyToSettle = 2,
    /// The proposal has been taken into account in a reward event.
    Settled = 3,
    /// The proposal is not eligible to be taken into account in a reward event.
    Ineligible = 4,
}

#[derive(AsRefStr, Debug, EnumString, ToPrimitive)]
#[strum(serialize_all = "snake_case")]
pub enum ProposalStatus {
    Unspecified = 0,
    /// A decision (adopt/reject) has yet to be made.
    Open = 1,
    /// The proposal has been rejected.
    Rejected = 2,
    /// The proposal has been adopted (sometimes also called
    /// "accepted"). At this time, either execution as not yet started,
    /// or it has but the outcome is not yet known.
    Adopted = 3,
    /// The proposal was adopted and successfully executed.
    Executed = 4,
    /// The proposal was adopted, but execution failed.
    Failed = 5,
}

#[derive(AsRefStr, Debug, EnumString, ToPrimitive)]
#[strum(serialize_all = "snake_case")]
pub enum Topic {
    Unspecified = 0,
    NeuronManagement = 1,
    ExchangeRate = 2,
    NetworkEconomics = 3,
    Governance = 4,
    NodeAdmin = 5,
    ParticipantManagement = 6,
    SubnetManagement = 7,
    NetworkCanisterManagement = 8,
    Kyc = 9,
    NodeProviderRewards = 10,
}

pub fn ledger_canister_id() -> Principal {
    Principal::from_slice(ic_nns_constants::LEDGER_CANISTER_ID.as_ref())
}

pub fn governance_canister_id() -> Principal {
    Principal::from_slice(ic_nns_constants::GOVERNANCE_CANISTER_ID.as_ref())
}

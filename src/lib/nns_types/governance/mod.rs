use ic_types::Principal;

pub fn ledger_canister_id() -> Principal {
    Principal::from_slice(ic_nns_constants::LEDGER_CANISTER_ID.as_ref())
}

pub fn governance_canister_id() -> Principal {
    Principal::from_slice(ic_nns_constants::GOVERNANCE_CANISTER_ID.as_ref())
}

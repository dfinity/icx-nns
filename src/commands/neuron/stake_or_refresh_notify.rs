use crate::lib::agent::create_waiter;
use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::{governance_canister_id, ledger_canister_id};
use crate::lib::nns_types::utils::{
    get_governance_subaccount, icpts_amount_validator, icpts_from_str,
};
use crate::lib::segregated_sign_send::sign::{sign_message, CanisterPayload, SignPayload};

use anyhow::anyhow;
use candid::{Decode, Encode};
use clap::Clap;
use ic_base_types::{CanisterId, PrincipalId};
use ic_nns_common::pb::v1::NeuronId;
use ic_types::Principal;
use ledger_canister::{BlockHeight, ICPTs, Memo, NotifyCanisterArgs, Subaccount, TRANSACTION_FEE};
use std::convert::TryFrom;

const NOTIFY_METHOD: &str = "notify_dfx";

/// Stake a new neuron or refresh an existing neuron
#[derive(Clap)]
pub struct StakeRefreshNeuronNotifyOpts {
    /// Specify the controller of the neuron
    controller: String,

    /// A unique numerical memo, or a nonce, associated
    /// with the neuron. To refresh an existing neuron,
    /// specify the same memo provided when it was created.
    memo: u64,

    /// The ledger block height at which the transaction was sent.
    block_height: u64,

    /// Max fee, default is 10000 e8s.
    #[clap(long, validator(icpts_amount_validator))]
    max_fee: Option<String>,
}

async fn notify(
    env: Env,
    block_height: BlockHeight,
    to_subaccount: Option<Subaccount>,
    max_fee: ICPTs,
    maybe_sign_payload: Option<SignPayload>,
) -> NnsCliResult<()> {
    let ledger_canister_id = ledger_canister_id();

    let governance_canister_id = governance_canister_id();

    let gov_base_types_principal = PrincipalId::try_from(governance_canister_id.clone().as_slice())
        .map_err(|err| anyhow!(err))?;

    let arg = Encode!(&NotifyCanisterArgs {
        block_height,
        max_fee,
        from_subaccount: None,
        to_canister: CanisterId::try_from(gov_base_types_principal).map_err(|err| anyhow!(err))?,
        to_subaccount,
    })?;

    match maybe_sign_payload {
        Some(payload) => {
            let mut sign_payload = payload;
            sign_payload.payload = Some(CanisterPayload {
                canister_id: ledger_canister_id,
                method_name: NOTIFY_METHOD.to_string(),
                is_query: false,
                arg,
            });
            sign_message(sign_payload, env.agent, env.sender).await?;
        }
        None => {
            let result = env
                .agent
                .update(&ledger_canister_id, NOTIFY_METHOD)
                .with_arg(arg)
                .call_and_wait(create_waiter())
                .await?;

            let neuron_id = Decode!(&result, NeuronId)?;
            println!("Neuron id: {:?}", neuron_id);
        }
    }

    Ok(())
}

pub async fn exec(
    opts: StakeRefreshNeuronNotifyOpts,
    maybe_sign_payload: Option<SignPayload>,
    env: Env,
) -> NnsCliResult {
    let memo = Memo(opts.memo);
    let block_height: BlockHeight = opts.block_height;

    let base_types_principal =
        PrincipalId::try_from(Principal::from_text(opts.controller)?.as_slice())
            .map_err(|err| anyhow!(err))?;

    let gov_subaccount = get_governance_subaccount(memo, base_types_principal);

    let to_subaccount = Some(gov_subaccount);

    let max_fee = opts
        .max_fee
        .map_or(Ok(TRANSACTION_FEE), |v| icpts_from_str(&v))
        .map_err(|err| anyhow!(err))?;

    notify(
        env,
        block_height,
        to_subaccount,
        max_fee,
        maybe_sign_payload,
    )
    .await?;
    Ok(())
}

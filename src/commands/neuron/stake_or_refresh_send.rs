use crate::lib::agent::create_waiter;
use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::{governance_canister_id, ledger_canister_id};
use crate::lib::nns_types::utils::{
    get_governance_subaccount, get_icpts_from_args, icpts_amount_validator, icpts_from_str,
};
use crate::lib::segregated_sign_send::sign::{sign_message, CanisterPayload, SignPayload};

use anyhow::anyhow;
use candid::{Decode, Encode};
use clap::Clap;
use ic_base_types::PrincipalId;
use ic_types::Principal;
use ledger_canister::{
    AccountIdentifier, BlockHeight, ICPTs, Memo, SendArgs, Subaccount, TRANSACTION_FEE,
};
use std::convert::TryFrom;

const SEND_METHOD: &str = "send_dfx";

/// Stake a new neuron or refresh an existing neuron
#[derive(Clap)]
pub struct StakeRefreshNeuronSendOpts {
    /// Specify the controller of the neuron
    controller: String,

    /// A unique numerical memo, or a nonce, associated
    /// with the neuron. To refresh an existing neuron,
    /// specify the same memo provided when it was created.
    memo: u64,

    /// ICP to stake or refresh into a neuron
    /// Can be specified as a Decimal with the fractional portion up to 8 decimal places
    /// i.e. 100.012
    #[clap(long, validator(icpts_amount_validator))]
    #[clap(long)]
    amount: Option<String>,

    /// Specify ICP as a whole number, helpful for use in conjunction with `--e8s`
    #[clap(long, conflicts_with("amount"))]
    icp: Option<u64>,

    /// Specify e8s as a whole number, helpful for use in conjunction with `--icp`
    #[clap(long, conflicts_with("amount"))]
    e8s: Option<u64>,

    /// Transaction fee, default is 10000 e8s.
    #[clap(long, validator(icpts_amount_validator))]
    fee: Option<String>,
}

async fn send(
    env: Env,
    memo: Memo,
    amount: ICPTs,
    fee: ICPTs,
    to_subaccount: Option<Subaccount>,
    maybe_sign_payload: Option<SignPayload>,
) -> NnsCliResult<()> {
    let ledger_canister_id = ledger_canister_id();
    let governance_canister_id = governance_canister_id();

    let gov_base_types_principal = PrincipalId::try_from(governance_canister_id.clone().as_slice())
        .map_err(|err| anyhow!(err))?;

    let to = AccountIdentifier::new(gov_base_types_principal, to_subaccount);

    let arg = Encode!(&SendArgs {
        memo,
        amount,
        fee,
        from_subaccount: None,
        to,
        created_at_time: None,
    })?;

    match maybe_sign_payload {
        Some(payload) => {
            let mut sign_payload = payload;
            sign_payload.payload = Some(CanisterPayload {
                canister_id: ledger_canister_id,
                method_name: SEND_METHOD.to_string(),
                is_query: false,
                arg,
            });
            sign_message(sign_payload, env.agent, env.sender).await?;
        }
        None => {
            let result = env
                .agent
                .update(&ledger_canister_id, SEND_METHOD)
                .with_arg(arg)
                .call_and_wait(create_waiter())
                .await?;

            let block_height = Decode!(&result, BlockHeight)?;
            println!("Transfer sent at BlockHeight: {}", block_height);
        }
    }

    Ok(())
}

pub async fn exec(
    opts: StakeRefreshNeuronSendOpts,
    maybe_sign_payload: Option<SignPayload>,
    env: Env,
) -> NnsCliResult {
    let amount = get_icpts_from_args(opts.amount, opts.icp, opts.e8s)?;

    let fee = opts
        .fee
        .map_or(Ok(TRANSACTION_FEE), |v| icpts_from_str(&v))
        .map_err(|err| anyhow!(err))?;

    let memo = Memo(opts.memo);

    let base_types_principal =
        PrincipalId::try_from(Principal::from_text(opts.controller)?.as_slice())
            .map_err(|err| anyhow!(err))?;

    let gov_subaccount = get_governance_subaccount(memo, base_types_principal);

    let to_subaccount = Some(gov_subaccount);

    send(env, memo, amount, fee, to_subaccount, maybe_sign_payload).await?;

    Ok(())
}

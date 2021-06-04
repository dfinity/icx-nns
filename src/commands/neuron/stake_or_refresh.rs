use crate::lib::agent::create_waiter;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::{governance_canister_id, ledger_canister_id};
use crate::lib::nns_types::utils::{
    get_governance_subaccount, get_icpts_from_args, icpts_amount_validator, icpts_from_str,
};

use anyhow::anyhow;
use candid::{Decode, Encode};
use clap::Clap;
use ic_agent::Agent;
use ic_base_types::{CanisterId, PrincipalId};
use ic_nns_common::pb::v1::NeuronId;
use ic_types::Principal;
use ledger_canister::{
    AccountIdentifier, BlockHeight, ICPTs, Memo, NotifyCanisterArgs, SendArgs, Subaccount,
    TRANSACTION_FEE,
};
use std::convert::TryFrom;

const SEND_METHOD: &str = "send_dfx";
const NOTIFY_METHOD: &str = "notify_dfx";

/// Stake a new neuron or refresh an existing neuron
#[derive(Clap)]
pub struct StakeRefreshNeuronOpts {
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

    /// Max fee, default is 10000 e8s.
    #[clap(long, validator(icpts_amount_validator))]
    max_fee: Option<String>,
}

async fn send_and_notify(
    agent: Agent,
    memo: Memo,
    amount: ICPTs,
    fee: ICPTs,
    to_subaccount: Option<Subaccount>,
    max_fee: ICPTs,
) -> NnsCliResult<NeuronId> {
    let ledger_canister_id = ledger_canister_id();

    let governance_canister_id = governance_canister_id();

    agent.fetch_root_key().await?;

    let gov_base_types_principal = PrincipalId::try_from(governance_canister_id.clone().as_slice())
        .map_err(|err| anyhow!(err))?;

    let to = AccountIdentifier::new(gov_base_types_principal, to_subaccount);

    let result = agent
        .update(&ledger_canister_id, SEND_METHOD)
        .with_arg(Encode!(&SendArgs {
            memo,
            amount,
            fee,
            from_subaccount: None,
            to,
            created_at_time: None,
        })?)
        .call_and_wait(create_waiter())
        .await?;

    let block_height = Decode!(&result, BlockHeight)?;
    println!("Transfer sent at BlockHeight: {}", block_height);

    let result = agent
        .update(&ledger_canister_id, NOTIFY_METHOD)
        .with_arg(Encode!(&NotifyCanisterArgs {
            block_height,
            max_fee,
            from_subaccount: None,
            to_canister: CanisterId::try_from(gov_base_types_principal)
                .map_err(|err| anyhow!(err))?,
            to_subaccount,
        })?)
        .call_and_wait(create_waiter())
        .await?;

    let result = Decode!(&result, NeuronId)?;
    Ok(result)
}

pub async fn exec(opts: StakeRefreshNeuronOpts, agent: Agent) -> NnsCliResult {
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

    let max_fee = opts
        .max_fee
        .map_or(Ok(TRANSACTION_FEE), |v| icpts_from_str(&v))
        .map_err(|err| anyhow!(err))?;

    let result = send_and_notify(agent, memo, amount, fee, to_subaccount, max_fee).await?;
    println!("Neuron id: {:?}", result);
    Ok(())
}

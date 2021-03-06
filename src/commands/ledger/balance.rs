use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::nns_types::governance::ledger_canister_id;

use anyhow::anyhow;
use candid::{Decode, Encode};
use clap::Clap;
use ic_base_types::PrincipalId;
use ledger_canister::{AccountBalanceArgs, AccountIdentifier, ICPTs};
use std::convert::TryFrom;
use std::str::FromStr;

const ACCOUNT_BALANCE_METHOD: &str = "account_balance_dfx";

/// Prints the account balance of the user
#[derive(Clap)]
pub struct BalanceOpts {
    /// Specifies an AccountIdentifier to get the balance of
    of: Option<String>,
}

pub async fn exec(opts: BalanceOpts, env: Env) -> NnsCliResult {
    let base_types_principal =
        PrincipalId::try_from(env.sender.as_slice()).map_err(|err| anyhow!(err))?;
    let acc_id = opts
        .of
        .map_or_else(
            || Ok(AccountIdentifier::new(base_types_principal, None)),
            |v| AccountIdentifier::from_str(&v),
        )
        .map_err(|err| anyhow!(err))?;

    let ledger_canister_id = ledger_canister_id();

    let arg = Encode!(&AccountBalanceArgs { account: acc_id })?;

    let result = env
        .agent
        .query(&ledger_canister_id, ACCOUNT_BALANCE_METHOD)
        .with_arg(arg)
        .call()
        .await?;

    let balance = Decode!(&result, ICPTs)?;

    println!("{}", balance);

    Ok(())
}

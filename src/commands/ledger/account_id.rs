use crate::lib::error::NnsCliResult;

use anyhow::anyhow;
use clap::Clap;
use ic_base_types::PrincipalId;
use ic_types::Principal;
use ledger_canister::AccountIdentifier;
use std::convert::TryFrom;

/// Prints the selected identity's AccountIdentifier.
#[derive(Clap)]
pub struct AccountIdOpts {}

pub async fn exec(_opts: AccountIdOpts, sender: Principal) -> NnsCliResult {
    let base_types_principal =
        PrincipalId::try_from(sender.as_slice()).map_err(|err| anyhow!(err))?;
    println!("{}", AccountIdentifier::new(base_types_principal, None));
    NnsCliResult::Ok(())
}

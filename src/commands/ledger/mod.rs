use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;
use crate::lib::segregated_sign_send::sign::SignPayload;

use clap::Clap;

mod account_id;
mod balance;

/// Call the ledger canister
#[derive(Clap)]
#[clap(name("ledger"))]
pub struct LedgerOpts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    AccountId(account_id::AccountIdOpts),
    Balance(balance::BalanceOpts),
}

pub async fn exec(
    opts: LedgerOpts,
    maybe_sign_payload: Option<SignPayload>,
    env: Env,
) -> NnsCliResult {
    match opts.subcmd {
        SubCommand::AccountId(v) => account_id::exec(v, env).await,
        SubCommand::Balance(v) => balance::exec(v, maybe_sign_payload, env).await,
    }
}

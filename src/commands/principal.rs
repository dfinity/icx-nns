use crate::lib::env::Env;
use crate::lib::error::NnsCliResult;

use clap::Clap;

/// Prints the selected identity's principal
#[derive(Clap)]
pub struct GetPrincipalOpts {}

pub async fn exec(_opts: GetPrincipalOpts, env: Env) -> NnsCliResult {
    println!("{}", env.sender.to_text());
    NnsCliResult::Ok(())
}

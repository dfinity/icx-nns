use crate::lib::error::NnsCliResult;

use clap::Clap;
use ic_types::Principal;

/// Prints the selected identity's principal
#[derive(Clap)]
pub struct GetPrincipalOpts {}

pub async fn exec(_opts: GetPrincipalOpts, sender: Principal) -> NnsCliResult {
    println!("{}", sender.to_text());
    NnsCliResult::Ok(())
}

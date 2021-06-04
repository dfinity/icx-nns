use crate::lib::agent::construct_agent;
use crate::lib::env::Env;
use crate::lib::identity::create_identity;
use clap::{AppSettings, Clap};

use anyhow::anyhow;
use tokio::runtime::Runtime;

mod commands;
mod lib;

const IC_ENDPOINT: &str = "https://ic0.app";

/// A tool to interact with IC NNS.
#[derive(Clap)]
#[clap(name("icx-nns"), global_setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    command: commands::Command,

    /// An IP address and port to connect to "<address>:<port>"
    #[clap(long)]
    endpoint: Option<String>,

    /// A flag to control whether or not to use the HSM backed identity
    #[clap(long)]
    use_hsm: bool,
}

fn main() {
    let opts = Opts::parse();
    let command = opts.command;
    let (network, fetch_root_key) = opts
        .endpoint
        .clone()
        .map_or((IC_ENDPOINT.to_string(), false), |v| {
            (format!("http://{}", v), true)
        });
    let use_hsm = opts.use_hsm;

    let runtime = Runtime::new().expect("Unable to create a runtime");

    let result = runtime.block_on(async {
        let identity = create_identity(use_hsm)?;
        let sender = identity.sender().map_err(|err| anyhow!("{}", err))?;

        let agent = construct_agent(identity, network.clone(), fetch_root_key).await?;

        let env = Env { agent, sender };

        commands::exec(command, env).await
    });

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(255);
    }
}

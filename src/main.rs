use crate::lib::agent::construct_agent;
use crate::lib::env::Env;
use crate::lib::identity::create_identity;
use crate::lib::segregated_sign_send::sign::SignPayload;
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

    /// Sign the message locally and save content to filename as specified
    /// with the `--flag` option. This file can be sent with `icx-nns send <file>`
    #[clap(long)]
    sign: bool,

    /// Specify the output file name. Extension type should be `.json`
    /// Default filename: message.json
    #[clap(long, requires("sign"))]
    file: Option<String>,

    /// Specifies how long will the message be valid in seconds
    /// Default expiry timeout: 5 minutes
    #[clap(long, requires("sign"))]
    expire_after: Option<String>,
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
    let sign = opts.sign;
    let file = opts.file.unwrap_or_else(|| "message.json".to_string());
    let expire_after = opts.expire_after.unwrap_or_else(|| "5m".to_string());

    let runtime = Runtime::new().expect("Unable to create a runtime");

    let result = runtime.block_on(async {
        let identity = create_identity(use_hsm)?;
        let sender = identity.sender().map_err(|err| anyhow!("{}", err))?;

        let agent = construct_agent(identity, network.clone(), fetch_root_key).await?;

        let env = Env { agent, sender };

        let maybe_sign_payload = if sign {
            Some(SignPayload {
                payload: None,
                network,
                expire_after,
                file,
            })
        } else {
            None
        };
        commands::exec(command, maybe_sign_payload, env).await
    });

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(255);
    }
}

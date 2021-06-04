use clap::{AppSettings, Clap};

mod commands;
mod lib;

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
    let endpoint = opts.endpoint;
    let use_hsm = opts.use_hsm;
    let result = commands::exec(command, endpoint, use_hsm);
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(255);
    }
}

use crate::lib::error::NnsCliResult;

use anyhow::{anyhow, bail};
use clap::Clap;
use ic_agent::Agent;

mod dissolve;
mod full_neuron;
mod hot_key;
mod ids;
mod info;
mod list;
mod stake_or_refresh;

/// Manage neuron subcommand
#[derive(Clap)]
#[clap(name("neuron"))]
pub struct NeuronOpts {
    #[clap(subcommand)]
    subcmd: SubCommand,
    id: Option<u64>,
}

#[derive(Clap)]
enum SubCommand {
    FullInfo(full_neuron::GetFullNeuronOpts),
    HotKey(hot_key::HotKeyOpts),
    Ids(ids::GetNeuronIdOpts),
    Info(info::GetNeuronInfoOpts),
    List(list::ListNeuronsOpts),
    StakeOrRefresh(stake_or_refresh::StakeRefreshNeuronOpts),
    Dissolve(dissolve::DissolveOpts),
}

pub async fn exec(opts: NeuronOpts, agent: Agent) -> NnsCliResult {
    let id = match opts.subcmd {
        SubCommand::FullInfo(_)
        | SubCommand::HotKey(_)
        | SubCommand::Info(_)
        | SubCommand::Dissolve(_) => opts.id.ok_or_else(|| {
            anyhow!("Please specify a neuron id i.e. `icx-nns neuron <id> <SUBCOMMAND>")
        })?,
        SubCommand::Ids(_) | SubCommand::List(_) | SubCommand::StakeOrRefresh(_) => {
            if let Some(id) = opts.id {
                bail!("Provided neuron id {} which is not needed for this command. Omit the neuron id and execute the command again", id);
            } else {
                0_u64 // unused
            }
        }
    };

    match opts.subcmd {
        SubCommand::FullInfo(v) => full_neuron::exec(v, id, agent).await,
        SubCommand::HotKey(v) => hot_key::exec(v, id, agent).await,
        SubCommand::Ids(v) => ids::exec(v, agent).await,
        SubCommand::Info(v) => info::exec(v, id, agent).await,
        SubCommand::List(v) => list::exec(v, agent).await,
        SubCommand::StakeOrRefresh(v) => stake_or_refresh::exec(v, agent).await,
        SubCommand::Dissolve(v) => dissolve::exec(v, id, agent).await,
    }
}

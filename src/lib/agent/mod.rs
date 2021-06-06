use crate::lib::error::NnsCliResult;

use anyhow::anyhow;
use garcon::Delay;
use ic_agent::{Agent, Identity};

pub fn create_waiter() -> Delay {
    Delay::builder()
        .throttle(std::time::Duration::from_secs(1))
        .build()
}

pub async fn construct_agent(
    identity: Box<dyn Identity + Send + Sync>,
    endpoint: String,
    fetch_root_key: bool,
) -> NnsCliResult<Agent> {
    let agent = Agent::builder()
        .with_url(endpoint)
        .with_boxed_identity(identity)
        .build()
        .map_err(|err| anyhow!("{:?}", err.to_string()))?;
    if fetch_root_key {
        let _ = agent.fetch_root_key().await?;
    }
    Ok(agent)
}

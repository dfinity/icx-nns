use crate::lib::error::NnsCliResult;
use crate::lib::identity::create_identity;

use anyhow::anyhow;
use garcon::Delay;
use ic_agent::export::Principal;
use ic_agent::Agent;

const IC_ENDPOINT: &str = "https://ic0.app";

pub fn create_waiter() -> Delay {
    Delay::builder()
        .throttle(std::time::Duration::from_secs(1))
        .build()
}

/// Constructs an `Agent` to be used for submitting requests.
pub async fn construct_agent(
    endpoint: Option<String>,
    use_hsm: bool,
) -> NnsCliResult<(Agent, Principal)> {
    let (url_str, fetch_root_key) = match endpoint {
        Some(endpoint) => (format!("http://{}", endpoint), true),
        None => (IC_ENDPOINT.to_string(), false),
    };

    let identity = create_identity(use_hsm).await?;
    let sender = identity.sender().map_err(|err| anyhow!("{}", err))?;
    let agent = Agent::builder()
        .with_url(url_str)
        .with_boxed_identity(identity)
        .build()
        .map_err(|err| anyhow!("{:?}", err.to_string()))?;
    if fetch_root_key {
        let _ = agent.fetch_root_key().await?;
    }
    Ok((agent, sender))
}

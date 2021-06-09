use ic_agent::Agent;
use ic_types::Principal;

pub struct Env {
    pub agent: Agent,
    pub sender: Principal,
}

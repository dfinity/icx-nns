use crate::lib::error::NnsCliResult;
use crate::lib::segregated_sign_send::sign_transport::SignReplicaV2Transport;
use crate::lib::segregated_sign_send::signed_message::SignedMessageV1;

use anyhow::{anyhow, bail};
use chrono::Utc;
use humanize_rs::duration;
use ic_agent::RequestId;
use ic_agent::{Agent, AgentError};
use ic_types::Principal;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;

pub struct CanisterPayload {
    pub canister_id: Principal,
    pub method_name: String,
    pub is_query: bool,
    pub arg: Vec<u8>,
}

pub struct SignPayload {
    pub payload: Option<CanisterPayload>,
    pub network: String,
    pub expire_after: String,
    pub file: String,
}

pub async fn sign_message(opts: SignPayload, agent: Agent, sender: Principal) -> NnsCliResult {
    let payload = opts.payload.unwrap();
    let timeout = duration::parse(&opts.expire_after)
        .map_err(|_| anyhow!("Cannot parse expire_after as a duration (e.g. `1h`, `1h 30m`)"))?;
    let expiration_system_time = SystemTime::now()
        .checked_add(timeout)
        .ok_or_else(|| anyhow!("Time wrapped around."))?;
    let chorono_timeout = chrono::Duration::seconds(timeout.as_secs() as i64);
    let creation = Utc::now();
    let expiration = creation
        .checked_add_signed(chorono_timeout)
        .ok_or_else(|| anyhow!("Expiration datetime overflow."))?;

    let message_template = SignedMessageV1::new(
        creation,
        expiration,
        opts.network,
        sender,
        payload.canister_id.clone(),
        payload.method_name.to_string(),
        payload.arg.clone(),
    );

    let file_name = opts.file;
    if Path::new(&file_name).exists() {
        bail!(
            "[{}] already exists, please specify a different output file name.",
            file_name
        );
    }

    let mut sign_agent = agent.clone();
    sign_agent.set_transport(SignReplicaV2Transport::new(
        file_name.clone(),
        message_template,
    ));

    if payload.is_query {
        let res = sign_agent
            .query(&payload.canister_id, payload.method_name)
            .with_effective_canister_id(payload.canister_id)
            .with_arg(&payload.arg)
            .expire_at(expiration_system_time)
            .call()
            .await;
        match res {
            Err(AgentError::TransportError(b)) => {
                println!("{}", b);
                Ok(())
            }
            Err(e) => bail!(e),
            Ok(_) => unreachable!(),
        }
    } else {
        let res = sign_agent
            .update(&payload.canister_id, payload.method_name)
            .with_effective_canister_id(payload.canister_id.clone())
            .with_arg(&payload.arg)
            .expire_at(expiration_system_time)
            .call()
            .await;
        match res {
            Err(AgentError::TransportError(b)) => {
                println!("{}", b);
                //Ok(())
            }
            Err(e) => bail!(e),
            Ok(_) => unreachable!(),
        }
        let path = Path::new(&file_name);
        let mut file = File::open(&path).map_err(|_| anyhow!("Message file doesn't exist."))?;
        let mut json = String::new();
        file.read_to_string(&mut json)
            .map_err(|_| anyhow!("Cannot read the message file."))?;
        let message: SignedMessageV1 =
            serde_json::from_str(&json).map_err(|_| anyhow!("Invalid json message."))?;
        // message from file guaranteed to have request_id becase it is a update message just generated
        let request_id = RequestId::from_str(&message.request_id.unwrap())?;
        let res = sign_agent
            .request_status_raw(&request_id, payload.canister_id.clone())
            .await;
        match res {
            Err(AgentError::TransportError(b)) => {
                println!("{}", b);
                Ok(())
            }
            Err(e) => bail!(e),
            Ok(_) => unreachable!(),
        }
    }
}

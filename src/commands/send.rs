use crate::lib::error::{NnsCliError, NnsCliResult};
use crate::lib::segregated_sign_send::signed_message::SignedMessageV1;

use anyhow::{anyhow, bail};
use clap::Clap;
use ic_agent::agent::{
    lookup_read_state_response, AgentError, QueryResponse, ReplicaV2Transport,
    RequestStatusResponse,
};
use ic_agent::{agent::http_transport::ReqwestHttpReplicaV2Transport, RequestId};
use ic_types::Principal;
use std::{fs::File, path::Path};
use std::{io::Read, str::FromStr};

/// Send a signed message
#[derive(Clap)]
pub struct SendOpts {
    /// Specifies the file name of the message
    file_name: String,

    /// Send the signed request-status call in the message
    #[clap(long)]
    status: bool,
}

pub async fn exec(opts: SendOpts) -> NnsCliResult {
    let file_name = opts.file_name;
    let path = Path::new(&file_name);
    let mut file = File::open(&path).map_err(|_| anyhow!("Message file doesn't exist."))?;
    let mut json = String::new();
    file.read_to_string(&mut json)
        .map_err(|_| anyhow!("Cannot read the message file."))?;
    let message: SignedMessageV1 =
        serde_json::from_str(&json).map_err(|_| anyhow!("Invalid json message."))?;
    message.validate()?;

    let network = message.network.clone();
    let transport = ReqwestHttpReplicaV2Transport::create(network)?;
    let content = hex::decode(&message.content)?;
    let canister_id = Principal::from_text(message.canister_id.clone())?;

    if opts.status {
        if message.call_type.clone().as_str() != "update" {
            bail!("Can only check request_status on update calls.");
        }
        if message.signed_request_status.is_none() {
            bail!("No signed_request_status in [{}].", file_name);
        }
        let envelope = hex::decode(&message.signed_request_status.unwrap())?;
        let response = transport.read_state(canister_id.clone(), envelope).await?;
        let request_id = RequestId::from_str(
            &message
                .request_id
                .expect("Cannot get request_id from the update message."),
        )?;
        match lookup_read_state_response(response, request_id).map_err(|err| anyhow!("{}", err))? {
            RequestStatusResponse::Replied { reply } => {
                let ic_agent::agent::Replied::CallReplied(blob) = reply;
                println!("{}", candid::IDLArgs::from_bytes(&blob)?);
            }
            RequestStatusResponse::Rejected {
                reject_code,
                reject_message,
            } => {
                return Err(NnsCliError::new(AgentError::ReplicaError {
                    reject_code,
                    reject_message,
                }))
            }
            RequestStatusResponse::Unknown => (),
            RequestStatusResponse::Received | RequestStatusResponse::Processing => {
                eprintln!("The update call has been received and is processing.")
            }
            RequestStatusResponse::Done => {
                return Err(NnsCliError::new(AgentError::RequestStatusDoneNoReply(
                    String::from(request_id),
                )))
            }
        }
        return Ok(());
    }

    eprintln!("Will send message:");
    eprintln!("  Creation:    {}", message.creation);
    eprintln!("  Expiration:  {}", message.expiration);
    eprintln!("  Network:     {}", message.network);
    eprintln!("  Call type:   {}", message.call_type);
    eprintln!("  Sender:      {}", message.sender);
    eprintln!("  Canister id: {}", message.canister_id);
    eprintln!("  Method name: {}", message.method_name);
    eprintln!("  Arg:         {:?}", message.arg);

    eprintln!("\nOkay? [y/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !["y", "yes"].contains(&input.to_lowercase().trim()) {
        return Ok(());
    }

    match message.call_type.as_str() {
        "query" => {
            let response: QueryResponse =
                serde_cbor::from_slice(&transport.query(canister_id, content).await?)
                    .map_err(|err| anyhow!("{}", err))?;
            match response {
                QueryResponse::Replied { reply } => {
                    println!("{}", candid::IDLArgs::from_bytes(&reply.arg)?);
                }
                QueryResponse::Rejected {
                    reject_code,
                    reject_message,
                } => {
                    eprintln!("{} {}", reject_code, reject_message);
                }
            };
        }
        "update" => {
            let request_id = RequestId::from_str(
                &message
                    .request_id
                    .expect("Cannot get request_id from the update message."),
            )?;
            transport
                .call(canister_id.clone(), content, request_id)
                .await?;

            eprintln!(
                "To check the status of this update call, append `--status` to current command."
            );
            eprintln!("e.g. `icx-nns send message.json --status`");
            eprint!("Request ID: ");
            println!("0x{}", String::from(request_id));
            eprint!("Canister ID: ");
            println!("{}", canister_id.to_string());
        }
        _ => unreachable!(),
    }
    Ok(())
}

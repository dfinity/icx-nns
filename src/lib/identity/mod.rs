use crate::lib::error::NnsCliResult;

use anyhow::anyhow;
use ic_agent::identity::BasicIdentity;
use ic_agent::Identity;
use ic_identity_hsm::HardwareIdentity;

const HSM_PKCS11_LIBRARY_PATH: &str = "HSM_PKCS11_LIBRARY_PATH";
const PEM_PATH: &str = "PEM_PATH";
const HSM_SLOT_INDEX: &str = "HSM_SLOT_INDEX";
const HSM_KEY_ID: &str = "HSM_KEY_ID";
const HSM_PIN: &str = "HSM_PIN";

fn expect_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("Need to specify the {} environment variable", name))
}

fn get_hsm_pin() -> Result<String, String> {
    expect_env_var(HSM_PIN)
}

fn create_basic_identity() -> NnsCliResult<Box<dyn Identity + Send + Sync>> {
    let id = match std::env::var(PEM_PATH) {
        Ok(_) => {
            let path = expect_env_var(PEM_PATH).map_err(|err| anyhow!("{}", err))?;
            BasicIdentity::from_pem_file(path).expect("Could not read the pem file.")
        }
        Err(_) => return Err(anyhow!("PEM_PATH environment variable is not set.")),
    };
    Ok(Box::new(id))
}

fn create_hsm_identity() -> NnsCliResult<Box<dyn Identity + Send + Sync>> {
    if std::env::var(HSM_PKCS11_LIBRARY_PATH).is_err() {
        return Err(anyhow!(
            "HSM_PKCS11_LIBRARY_PATH environment variable is not set."
        ));
    }
    let path = expect_env_var(HSM_PKCS11_LIBRARY_PATH).map_err(|err| anyhow!("{}", err))?;
    let slot_index = expect_env_var(HSM_SLOT_INDEX)
        .map_err(|err| anyhow!("{}", err))?
        .parse::<usize>()
        .map_err(|e| anyhow!("Unable to parse {} value: {}", HSM_SLOT_INDEX, e))?;
    let key = expect_env_var(HSM_KEY_ID).map_err(|err| anyhow!("{}", err))?;
    let id = HardwareIdentity::new(path, slot_index, &key, get_hsm_pin)
        .map_err(|e| anyhow!("Unable to create hw identity: {}", e))?;
    Ok(Box::new(id))
}

pub fn create_identity(use_hsm: bool) -> NnsCliResult<Box<dyn Identity + Send + Sync>> {
    if use_hsm {
        create_hsm_identity()
    } else {
        create_basic_identity()
    }
}

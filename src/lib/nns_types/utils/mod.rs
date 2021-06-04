use crate::lib::error::NnsCliResult;

use anyhow::anyhow;
use ic_base_types::PrincipalId;
use ledger_canister::{ICPTs, Memo, Subaccount, DECIMAL_PLACES};
use openssl::sha::Sha256;
use rust_decimal::Decimal;
use std::convert::TryFrom;
use std::str::FromStr;

pub fn icpts_amount_validator(icpts: &str) -> Result<(), String> {
    let err_message = format!("Could not convert {} to ICP type", icpts);
    icpts_from_str(icpts).map(|_| ()).map_err(|_| err_message)
}

pub fn get_governance_subaccount(memo: Memo, principal: PrincipalId) -> Subaccount {
    Subaccount::try_from(
        &{
            let mut state = Sha256::new();
            state.update(&[0x0c]);
            state.update(b"neuron-stake");
            state.update(&principal.as_slice());
            state.update(&memo.0.to_be_bytes());
            state.finish()
        }[..],
    )
    .expect("Couldn't build subaccount from hash.")
}

pub fn icpts_from_str(s: &str) -> NnsCliResult<ICPTs> {
    match Decimal::from_str(s) {
        Ok(amount) => {
            if amount.scale() > DECIMAL_PLACES {
                return Err(anyhow!(
                    "e8s can only be specified to the 8th decimal.".to_string()
                ));
            }
            let icpts = match amount.trunc().to_string().parse::<u64>() {
                Ok(v) => v,
                Err(e) => return Err(anyhow!(format!("{}", e))),
            };
            let e8s = match amount.fract().to_string().as_str() {
                "0" => 0_u64,
                e8s => {
                    let e8s = &e8s.to_string()[2..e8s.to_string().len()];
                    let amount = e8s.chars().enumerate().fold(0, |amount, (idx, val)| {
                        amount
                            + (10_u64.pow(DECIMAL_PLACES - 1 - (idx as u32))
                                * (val.to_digit(10).unwrap() as u64))
                    });
                    amount as u64
                }
            };
            ICPTs::new(icpts, e8s).map_err(|err| anyhow!(err))
        }
        Err(e) => Err(anyhow!(format!("Decimal conversion error: {}", e))),
    }
}

pub fn get_icpts_from_args(
    amount: Option<String>,
    icp: Option<u64>,
    e8s: Option<u64>,
) -> NnsCliResult<ICPTs> {
    if amount.is_none() {
        let icp = match icp {
            Some(v) => ICPTs::from_icpts(v).map_err(|err| anyhow!(err))?,
            None => ICPTs::from_e8s(0),
        };
        let icp_from_e8s = match e8s {
            Some(v) => ICPTs::from_e8s(v),
            None => ICPTs::from_e8s(0),
        };
        let amount = icp + icp_from_e8s;
        Ok(amount.map_err(|err| anyhow!(err))?)
    } else {
        Ok(icpts_from_str(&amount.unwrap())
            .map_err(|err| anyhow!("Could not add ICPs and e8s: {}", err))?)
    }
}

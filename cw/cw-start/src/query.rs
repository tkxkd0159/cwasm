use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Env, StdResult, Uint128};
use crate::state::{ContractInfo, CONTRACT_INFO};

#[cw_serde]
pub struct EnvResponse {
    pub env_info: Env,
    pub contract_info: ContractInfo
}

#[cw_serde]
pub struct PendingFundsResponse {
    pub amount: Uint128
}

pub fn q_env(deps: Deps, env: Env) -> StdResult<EnvResponse> {
    let cinfo = CONTRACT_INFO.load(deps.storage)?;

    Ok(EnvResponse {
        env_info: env,
        contract_info: cinfo
    })
}

pub fn q_pending_funds(_deps: Deps, _denom: String) -> StdResult<PendingFundsResponse> {
    Ok(PendingFundsResponse { amount: Uint128::from(0u128) })
}

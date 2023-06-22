use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Env, StdResult, Uint128};
use crate::state::{ContractInfo, CONTRACT_INFO, COUNTER};

#[cw_serde]
pub struct EnvResponse {
    pub env_info: Env,
    pub contract_info: ContractInfo
}

#[cw_serde]
pub struct CountResponse {
    count: u64
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

pub fn q_count(deps: Deps, owner: String) -> StdResult<CountResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let v = COUNTER.load(deps.storage, owner_addr)?;
    Ok(CountResponse{ count: v.count })
}

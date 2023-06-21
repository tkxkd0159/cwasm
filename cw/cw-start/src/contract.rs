use cosmwasm_std::{Binary, DepsMut, Deps, Env, MessageInfo, StdResult, Response, entry_point, to_binary, Event};
use crate::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{CONTRACT_INFO, ContractInfo};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    CONTRACT_INFO.save(deps.storage, &ContractInfo{owner: info.sender.to_string(), description: msg.desc.clone() })?;

    Ok(Response::default()
        .add_event(Event::new("contract_info").add_attributes(vec![
        ("owner", info.sender.to_string()),
        ("description", msg.desc)])
        )
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    use crate::msg::QueryMsg;
    use crate::query::{q_env, q_pending_funds};

    match msg {
        QueryMsg::Env {} => to_binary(&q_env(deps, env)?),
        QueryMsg::PendingFunds { denom } => to_binary(&q_pending_funds(deps, denom)?),
    }
}

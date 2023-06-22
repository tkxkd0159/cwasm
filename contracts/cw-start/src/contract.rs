use cosmwasm_std::{Binary, DepsMut, Deps, Env, MessageInfo, StdResult, Response, entry_point, to_binary, Event};
use crate::{ContractError};
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
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::exec::{exec_empty, exec_increment, exec_bank_send};

    match msg {
        ExecuteMsg::Empty {} => exec_empty(),
        ExecuteMsg::Increment {} => exec_increment(deps, info),
        ExecuteMsg::BankSend { receiver, amount } => exec_bank_send(receiver, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    use crate::query::{q_env, q_count, q_pending_funds};

    match msg {
        QueryMsg::Env {} => to_binary(&q_env(deps, env)?),
        QueryMsg::PendingFunds { denom } => to_binary(&q_pending_funds(deps, denom)?),
        QueryMsg::Count { owner } => to_binary(&q_count(deps, owner)?),
    }
}

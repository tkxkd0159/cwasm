use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractInfo, CONTRACT_INFO};
use crate::ContractError;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CONTRACT_INFO.save(
        deps.storage,
        &ContractInfo {
            owner: info.sender.to_string(),
            description: msg.desc.clone(),
        },
    )?;

    Ok(
        Response::default().add_event(Event::new("contract_info").add_attributes(vec![
            ("owner", info.sender.to_string()),
            ("description", msg.desc),
        ])),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::exec::{exec_bank_send, exec_empty, exec_increment};

    match msg {
        ExecuteMsg::Empty {} => exec_empty(),
        ExecuteMsg::Increment {} => exec_increment(deps, info),
        ExecuteMsg::BankSend { receiver, amount } => exec_bank_send(receiver, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use crate::query::{q_count, q_env, q_pending_funds};

    match msg {
        QueryMsg::Env {} => to_binary(&q_env(deps, env)?),
        QueryMsg::PendingFunds { denom } => to_binary(&q_pending_funds(deps, denom)?),
        QueryMsg::Count { owner } => to_binary(&q_count(deps, owner)?),
    }
}

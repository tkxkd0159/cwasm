use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ContractInfo, CONTRACT_INFO, OWNER};
use crate::ContractError;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "cw-start"; // env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::exec::{deposit, exec_bank_send, exec_empty, exec_increment, withdraw};

    match msg {
        ExecuteMsg::Empty {} => exec_empty(),
        ExecuteMsg::Increment {} => exec_increment(deps, info),
        ExecuteMsg::BankSend { receiver, amount } => exec_bank_send(receiver, amount),
        ExecuteMsg::Deposit {} => deposit(info),
        ExecuteMsg::Withdraw { recipient, amount } => withdraw(deps, env, info, recipient, amount),
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

use crate::error::ContractError;
use crate::state::{CounterState, COUNTER};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, MessageInfo, Response, StdResult};

pub fn exec_empty() -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn exec_increment(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let cnt = match COUNTER.may_load(deps.storage, info.sender.clone())? {
        Some(num) => {
            COUNTER.update(
                deps.storage,
                info.sender.clone(),
                |c: Option<CounterState>| -> StdResult<_> {
                    Ok(CounterState {
                        count: c.unwrap().count + 1,
                    })
                },
            )?;
            num.count + 1
        }
        None => {
            COUNTER.save(
                deps.storage,
                info.sender.clone(),
                &CounterState { count: 1 },
            )?;
            1
        }
    };

    Ok(Response::default()
        .add_attribute("owner", info.sender.clone())
        .add_attribute("count", cnt.to_string()))
}

pub fn exec_bank_send(receiver: String, amount: Vec<Coin>) -> Result<Response, ContractError> {
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: receiver,
        amount,
    });

    let res = Response::new().add_message(msg);
    Ok(res)
}

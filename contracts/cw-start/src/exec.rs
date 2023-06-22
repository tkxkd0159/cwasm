use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
use crate::error::ContractError;
use crate::state::{COUNTER, CounterState};

pub fn exec_empty() -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn exec_increment(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let cnt = match COUNTER.may_load(deps.storage, info.sender.clone())? {
        Some(num) => {
            COUNTER.update(deps.storage, info.sender.clone(), |c: Option<CounterState>|-> StdResult<_> {
                Ok(CounterState{count: c.unwrap().count + 1})
            })?;
            num.count+1
        }
        None => {
            COUNTER.save(deps.storage, info.sender.clone(), &CounterState{count: 1})?;
            1
        }
    };

    Ok(Response::default()
        .add_attribute("owner", info.sender.clone())
        .add_attribute("count", cnt.to_string())
    )
}

use crate::error::ContractError;
use crate::state::{CONTRACT_INFO, OWNER};
use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

pub fn exec_empty() -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn exec_increment(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let resp = CONTRACT_INFO.update(deps.storage, |mut state| -> StdResult<_> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::default()
        .add_attribute("owner", info.sender.clone())
        .add_attribute("count", resp.count.to_string()))
}

pub fn exec_bank_send(receiver: String, amount: Vec<Coin>) -> Result<Response, ContractError> {
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: receiver,
        amount,
    });

    let resp = Response::new().add_message(msg);
    Ok(resp)
}

pub fn deposit(info: MessageInfo) -> Result<Response, ContractError> {
    if info.funds == vec![] {
        return Err(ContractError::InsufficientFunds {});
    }

    Ok(Response::default())
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Std(StdError::generic_err("Unauthorized")));
    }

    let mut hit_num = 0;
    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    for coin in amount.iter() {
        for b in balance.iter() {
            if b.denom == coin.denom {
                if b.amount < coin.amount {
                    return Err(ContractError::InsufficientFunds {});
                }
                hit_num += 1;
                break;
            }
        }
    }

    if hit_num != amount.len() {
        return Err(ContractError::InsufficientFunds {});
    }

    let bank_msg = BankMsg::Send {
        to_address: recipient.clone(),
        amount: amount.clone(),
    };

    let amt_str = amount
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("sender", info.sender)
        .add_attribute("receiver", recipient)
        .add_attribute("amount", amt_str);

    Ok(resp)
}

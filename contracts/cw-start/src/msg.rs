use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[allow(unused_imports)]
use crate::query::{CountResponse, EnvResponse, PendingFundsResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub desc: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Empty {},
    Increment {},
    BankSend {
        receiver: String,
        amount: Vec<Coin>,
    },
    Deposit {},
    Withdraw {
        recipient: String,
        amount: Vec<Coin>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(EnvResponse)]
    Env {},

    #[returns(PendingFundsResponse)]
    PendingFunds { denom: String },

    #[returns(CountResponse)]
    Count { owner: String },
}

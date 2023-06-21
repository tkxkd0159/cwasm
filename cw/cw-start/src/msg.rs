use cosmwasm_schema::{cw_serde, QueryResponses};

#[allow(unused_imports)]
use crate::query::{EnvResponse, PendingFundsResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub desc: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Empty {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(EnvResponse)]
    Env {},

    #[returns(PendingFundsResponse)]
    PendingFunds { denom: String },
}

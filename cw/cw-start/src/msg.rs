use cosmwasm_schema::{cw_serde, QueryResponses};

#[allow(unused_imports)]
use crate::query::{EmptyResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Empty {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(EmptyResponse)]
    Empty {},
}

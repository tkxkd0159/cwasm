use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct ContractInfo {
    pub owner: String,
    pub description: String,
}

#[cw_serde]
pub struct CounterState {
    pub count: u64,
}

pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");
pub const COUNTER: Map<Addr, CounterState> = Map::new("counter");

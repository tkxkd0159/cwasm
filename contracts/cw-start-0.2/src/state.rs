use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct ContractInfo {
    pub owner: String,
    pub description: String,
    pub count: u64,
}

pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("custom_contract_info");
pub const OWNER: Item<Addr> = Item::new("owner");

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct ContractInfo {
    pub owner: String,
    pub description: String,
}

pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");

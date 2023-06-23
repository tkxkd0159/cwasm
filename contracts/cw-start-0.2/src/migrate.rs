use cosmwasm_std::{Addr, DepsMut, Order, Response, StdResult};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

use crate::state::{ContractInfo, CONTRACT_INFO};

pub fn migrate_0_1_0(deps: DepsMut) -> StdResult<Response> {
    #[derive(Serialize, Deserialize)]
    struct OldContractInfo {
        owner: String,
        description: String,
    }

    #[derive(Serialize, Deserialize)]
    struct OldCounterState {
        count: u64,
    }

    const OLD_CONTRACT_INFO: Item<OldContractInfo> = Item::new("cwstart_contract_info");
    const OLD_COUNTER: Map<Addr, OldCounterState> = Map::new("counter");

    let OldContractInfo { owner, description } = OLD_CONTRACT_INFO.load(deps.storage)?;

    let count_list = OLD_COUNTER
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.unwrap().1.count)
        .collect::<Vec<u64>>();

    let total_count: u64 = count_list.iter().sum();

    CONTRACT_INFO.save(
        deps.storage,
        &ContractInfo {
            owner,
            description,
            count: total_count,
        },
    )?;

    Ok(Response::default())
}

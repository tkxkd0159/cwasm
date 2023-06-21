use cosmwasm_schema::write_api;
use cw_start::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

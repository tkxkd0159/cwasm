pub mod contract;
mod error;
pub use crate::error::ContractError;
pub mod exec;
pub mod msg;
pub mod query;
pub mod state;

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::query::EnvResponse;
    use cosmwasm_std::{coins, Addr, Attribute, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor, Module};

    fn mock_contract() -> Box<dyn Contract<Empty>> {
        let c = ContractWrapper::new(execute, instantiate, query);
        Box::new(c)
    }

    #[test]
    fn query_env() {
        let mut app = App::default();
        let contract_id = app.store_code(mock_contract());
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    desc: "query_env".to_string(),
                },
                &[],
                "Starting contract",
                None,
            )
            .unwrap();

        let resp: EnvResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Env {})
            .unwrap();
        assert_eq!(resp.contract_info.description, "query_env");
        assert_eq!(resp.contract_info.owner, "sender");
        assert_eq!(resp.env_info.block.height, 12345);
        assert_eq!(resp.env_info.block.chain_id, "cosmos-testnet-14002");
        assert_eq!(resp.env_info.contract.address, contract_addr);
        assert_eq!(resp.env_info.transaction.unwrap().index, 0);
    }

    #[test]
    fn exec_increment() {
        let sender = Addr::unchecked("sender");
        let mut app = App::new(|router, api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "stake"))
                .unwrap();
        });
        let contract_id = app.store_code(mock_contract());
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg {
                    desc: "exec_increment".to_string(),
                },
                &[],
                "Starting contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                sender.clone(),
                contract_addr.clone(),
                &ExecuteMsg::Increment {},
                &[],
            )
            .unwrap();
        assert_eq!(resp.events[0].ty, "execute");
        assert_eq!(resp.events[1].ty, "wasm");
        assert_eq!(
            resp.events[1].attributes,
            vec![
                Attribute {
                    key: "_contract_addr".to_string(),
                    value: "contract0".to_string()
                },
                Attribute::new("owner", sender),
                Attribute::new("count", "1"),
            ]
        );
    }
}

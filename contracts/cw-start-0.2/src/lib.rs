pub mod contract;
mod error;
mod migrate;
pub use crate::error::ContractError;
pub mod exec;
pub mod msg;
pub mod query;
pub mod state;

#[cfg(any(test, feature = "tests"))]
mod multitest;

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use cosmwasm_std::{coins, Addr, Attribute, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    fn mock_contract() -> Box<dyn Contract<Empty>> {
        let c = ContractWrapper::new(execute, instantiate, query);
        Box::new(c)
    }

    #[test]
    fn exec_increment() {
        let sender = Addr::unchecked("sender");
        let mut app = App::new(|router, _api, storage| {
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
                Attribute::new("count", "2"),
            ]
        );
    }
}

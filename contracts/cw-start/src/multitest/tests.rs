use super::contract::MyContract;
use crate::error::ContractError;
use cosmwasm_std::{coins, Addr};
use cw_multi_test::App;

const DENOM: &str = "stakes";

#[test]
fn withdraw() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, DENOM))
            .unwrap();
    });
    let cid = MyContract::store_code(&mut app);
    let contract =
        MyContract::instantiate(&mut app, cid, &owner, &[], "test", None, "test").unwrap();

    let err = contract.depoist(&mut app, &sender, &[]).unwrap_err();
    assert_eq!(ContractError::InsufficientFunds {}, err);

    contract
        .depoist(&mut app, &sender, &coins(50, DENOM))
        .unwrap();
    contract
        .withdraw(&mut app, &owner, &[], &owner, &coins(17, DENOM))
        .unwrap();

    assert_eq!(
        MyContract::query_balances(&app, &owner).unwrap(),
        coins(17, DENOM)
    );
    assert_eq!(
        MyContract::query_balances(&app, &sender).unwrap(),
        coins(50, DENOM)
    );
    assert_eq!(
        MyContract::query_balances(&app, &contract.address()).unwrap(),
        coins(33, DENOM)
    );
}

#[test]
fn query_env() {
    let owner = Addr::unchecked("owner");
    let mut app = App::default();
    let cid = MyContract::store_code(&mut app);
    let contract =
        MyContract::instantiate(&mut app, cid, &owner, &[], "test", None, "query_env").unwrap();

    let resp = contract.query_env(&mut app).unwrap();
    assert_eq!(resp.contract_info.description, "query_env");
    assert_eq!(resp.contract_info.owner, "owner");
    assert_eq!(resp.env_info.block.height, 12345);
    assert_eq!(resp.env_info.block.chain_id, "cosmos-testnet-14002");
    assert_eq!(resp.env_info.contract.address, contract.address());
    assert_eq!(resp.env_info.transaction.unwrap().index, 0);
}

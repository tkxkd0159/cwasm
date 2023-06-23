use cosmwasm_std::{coins, from_slice, Addr};
use cw_multi_test::App;

use super::contract::MyContract;
use crate::error::ContractError;

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

#[test]
fn migration() {
    use cw_start_0_1::multitest::contract::MyContract as OldContract;
    use cw_start_0_1::state::ContractInfo as OldContractInfo;

    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, DENOM))
            .unwrap();
    });

    let old_code_id = OldContract::store_code(&mut app);
    let new_code_id = MyContract::store_code(&mut app);

    let initial_info = OldContractInfo {
        owner: owner.to_string(),
        description: "migrate".to_string(),
    };

    let old_contract = OldContract::instantiate(
        &mut app,
        old_code_id,
        &owner,
        &[],
        "test",
        Some(owner.to_string()),
        &initial_info.description,
    )
    .unwrap();
    old_contract.increment(&mut app, &sender, &[]).unwrap();
    old_contract.increment(&mut app, &owner, &[]).unwrap();

    let contract = MyContract::migrate(&mut app, old_contract.into(), new_code_id, &owner).unwrap();
    let resp = contract.query_env(&mut app).unwrap();
    assert_eq!(resp.contract_info.description, "migrate");
    assert_eq!(resp.contract_info.owner, owner.to_string());
    assert_eq!(resp.contract_info.count, 2);

    let v = app
        .wrap()
        .query_wasm_raw(contract.address(), "owner".as_bytes())
        .unwrap();
    assert_eq!(owner, from_slice::<Addr>(&v.unwrap()).unwrap());
}

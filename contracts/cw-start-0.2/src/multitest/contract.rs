use cosmwasm_std::StdResult;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use crate::contract::{execute, instantiate, migrate, query};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{CountResponse, EnvResponse, PendingFundsResponse};
use crate::ContractError;

#[derive(Debug)]
pub struct MyContract(Addr);

impl From<MyContract> for Addr {
    fn from(contract: MyContract) -> Self {
        contract.0
    }
}

impl MyContract {
    pub fn address(&self) -> Addr {
        self.0.clone()
    }

    #[track_caller]
    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query).with_migrate(migrate);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        funds: &[Coin],
        label: &str,
        admin: impl Into<Option<String>>,
        desc: &str,
    ) -> StdResult<Self> {
        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                desc: desc.to_string(),
            },
            funds,
            label,
            admin.into(),
        )
        .map(MyContract)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn migrate(app: &mut App, contract: Addr, code_id: u64, sender: &Addr) -> StdResult<Self> {
        app.migrate_contract(sender.clone(), contract.clone(), &MigrateMsg {}, code_id)
            .map(|_| Self(contract))
            .map_err(|err| err.downcast().unwrap())
    }
}

// Execute
impl MyContract {
    #[track_caller]
    pub fn increment(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<AppResponse, ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Increment {},
            funds,
        )
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn depoist(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<AppResponse, ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Deposit {},
            funds,
        )
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn withdraw(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
        recipient: &Addr,
        amount: &[Coin],
    ) -> Result<AppResponse, ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Withdraw {
                recipient: recipient.to_string(),
                amount: amount.to_vec(),
            },
            funds,
        )
        .map_err(|err| err.downcast().unwrap())
    }
}

// Query
impl MyContract {
    #[track_caller]
    pub fn query_balances(app: &App, owner: &Addr) -> StdResult<Vec<Coin>> {
        app.wrap().query_all_balances(owner)
    }

    #[track_caller]
    pub fn query_env(&self, app: &App) -> StdResult<EnvResponse> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Env {})
    }

    #[track_caller]
    pub fn query_pending_funds(&self, app: &App, denom: &str) -> StdResult<PendingFundsResponse> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::PendingFunds {
                denom: denom.to_string(),
            },
        )
    }

    #[track_caller]
    pub fn query_count(&self, app: &App) -> StdResult<CountResponse> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Count {})
    }
}

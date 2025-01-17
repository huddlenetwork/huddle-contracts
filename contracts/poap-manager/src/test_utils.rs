#![cfg(test)]
use cosmwasm_std::{
    Binary, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response, StdError, StdResult,
};
use cw721_base::{
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721ExecuteMsg,
    InstantiateMsg as Cw721InstantiateMsg, QueryMsg as Cw721QueryMsg,
};
use cw721_poap::Metadata;
use cw_multi_test::{Contract, ContractWrapper};
use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};
use poap::{
    contract::{
        execute as poap_execute, instantiate as poap_instantiate, query as poap_query,
        reply as poap_reply,
    },
    msg::{
        ExecuteMsg as POAPExecuteMsg, InstantiateMsg as POAPInstantiateMsg,
        QueryMsg as POAPQueryMsg,
    },
    ContractError as POAPContractError,
};

pub struct POAPTestContract;
impl POAPTestContract {
    fn instantiate(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: POAPInstantiateMsg,
    ) -> Result<Response<DesmosMsg>, POAPContractError> {
        poap_instantiate(deps, env, info, msg)
    }

    fn failing_instantiate(
        _deps: DepsMut<DesmosQuery>,
        _env: Env,
        _info: MessageInfo,
        _msg: Cw721InstantiateMsg,
    ) -> Result<Response<DesmosMsg>, POAPContractError> {
        Err(POAPContractError::Std(StdError::generic_err(
            "poap initialization failed",
        )))
    }

    fn execute(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: POAPExecuteMsg,
    ) -> Result<Response<DesmosMsg>, POAPContractError> {
        poap_execute(deps, env, info, msg)
    }

    fn query(deps: Deps<DesmosQuery>, env: Env, msg: POAPQueryMsg) -> StdResult<Binary> {
        poap_query(deps, env, msg)
    }

    fn reply(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        msg: Reply,
    ) -> Result<Response<DesmosMsg>, POAPContractError> {
        poap_reply(deps, env, msg)
    }

    /// Provides an instance of a poap contract.
    /// This instance can be used only during the integration tests.
    pub fn success_contract() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(Self::execute, Self::instantiate, Self::query)
            .with_reply(Self::reply);
        Box::new(contract)
    }

    /// Provides an instance of a poap contract that fails during the initialization.
    /// This instance can be used only during the integration tests.
    pub fn failing_contract() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(Self::execute, Self::failing_instantiate, Self::query);
        Box::new(contract)
    }
}

pub struct CW721TestContract;
impl CW721TestContract {
    fn instantiate(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: Cw721InstantiateMsg,
    ) -> Result<Response<DesmosMsg>, StdError> {
        Cw721Contract::<'static, Metadata, Empty, Empty, DesmosMsg, DesmosQuery>::default()
            .instantiate(deps, env, info, msg)
    }

    fn failing_instantiate(
        _deps: DepsMut<DesmosQuery>,
        _env: Env,
        _info: MessageInfo,
        _msg: Cw721InstantiateMsg,
    ) -> Result<Response<DesmosMsg>, StdError> {
        Err(StdError::generic_err("cw721 initialization failed"))
    }

    fn execute(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<Metadata, Empty>,
    ) -> Result<Response<DesmosMsg>, Cw721ContractError> {
        Cw721Contract::<'static, Metadata, Empty, Empty, DesmosMsg, DesmosQuery>::default()
            .execute(deps, env, info, msg)
    }

    fn query(deps: Deps<DesmosQuery>, env: Env, msg: Cw721QueryMsg<Empty>) -> StdResult<Binary> {
        Cw721Contract::<'static, Metadata, Empty, Empty, DesmosMsg, DesmosQuery>::default()
            .query(deps, env, msg)
    }

    /// Provides an instance of a cw721 contract.
    /// This instance can be used only during the integration tests.
    pub fn success_contract() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(Self::execute, Self::instantiate, Self::query);
        Box::new(contract)
    }

    /// Provides an instance of a cw721 contract that fails during the initialization.
    /// This instance can be used only during the integration tests.
    pub fn failing_contract() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(Self::execute, Self::failing_instantiate, Self::query);
        Box::new(contract)
    }
}

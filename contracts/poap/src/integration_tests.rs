#[cfg(test)]
mod tests {
    use crate::cw721_test_utils;
    use crate::msg::{
        ExecuteMsg, QueryConfigResponse, QueryEventInfoResponse, QueryMintedAmountResponse,
        QueryMsg,
    };
    use crate::test_utils::{
        get_valid_init_msg, ADMIN, CREATOR, EVENT_END_SECONDS, EVENT_START_SECONDS,
        INITIAL_BLOCK_TIME_SECONDS, MINTER, POAP_URI, USER,
    };
    use cosmwasm_std::{Addr, Empty, Timestamp, Uint64};
    use cw721::{AllNftInfoResponse, NftInfoResponse, OwnerOfResponse, TokensResponse};
    use cw721_base::{MinterResponse, QueryMsg as Cw721QueryMsg};
    use cw721_poap::Metadata;

    use cw_multi_test::{Contract, ContractWrapper, Executor};
    use desmos_bindings::{
        mocks::mock_apps::{mock_desmos_app, DesmosApp},
        msg::DesmosMsg,
        query::DesmosQuery,
    };

    fn contract_poap() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )
        .with_reply(crate::contract::reply);
        Box::new(contract)
    }

    fn mock_app() -> DesmosApp {
        mock_desmos_app()
    }

    /// Uploads the contracts to the app.
    /// Returns a pair of ids where the first refers to the cw721
    /// and the second to the poap.
    fn store_contracts(app: &mut DesmosApp) -> (u64, u64) {
        let cw721_code_id = app.store_code(cw721_test_utils::contract_cw721());
        let poap_code_id = app.store_code(contract_poap());

        (cw721_code_id, poap_code_id)
    }

    fn proper_instantiate() -> (DesmosApp, Addr) {
        let mut app = mock_app();
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS)
        });
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let msg = get_valid_init_msg(cw721_code_id);

        let poap_contract_addr = app
            .instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "poap_contract",
                None,
            )
            .unwrap();

        (app, poap_contract_addr)
    }

    #[test]
    fn instantiate_with_invalid_cw721_code_id_error() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        init_msg.cw721_code_id = 42u64.into();

        let init_result = app.instantiate_contract(
            poap_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "poap_contract",
            None,
        );
        assert!(init_result.is_err());
    }

    #[test]
    fn instantiate_with_failing_cw721_contract_error() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let failing_cw721_code_id = app.store_code(cw721_test_utils::failing_cw721());
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        init_msg.cw721_code_id = failing_cw721_code_id.into();

        let init_result = app.instantiate_contract(
            poap_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "poap_contract",
            None,
        );
        assert!(init_result.is_err());
    }

    #[test]
    fn proper_contracts_instantiation() {
        let (app, poap_contract_addr) = proper_instantiate();

        let querier = app.wrap();

        let poap_config: QueryConfigResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        assert_eq!(Addr::unchecked(ADMIN), poap_config.admin);
        assert_eq!(Addr::unchecked(MINTER), poap_config.minter);
        assert_eq!(false, poap_config.mint_enabled);
        // 1 since is the first uploaded.
        assert_eq!(Uint64::new(1), poap_config.cw721_contract_code);

        let poap_event_info: QueryEventInfoResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::EventInfo {})
            .unwrap();

        assert_eq!(Addr::unchecked(CREATOR), poap_event_info.creator);
        assert_eq!(
            Timestamp::from_seconds(EVENT_START_SECONDS),
            poap_event_info.start_time
        );
        assert_eq!(
            Timestamp::from_seconds(EVENT_END_SECONDS),
            poap_event_info.end_time
        );
        assert_eq!(POAP_URI, poap_event_info.poap_uri.as_str());

        let cw721_minter_response: MinterResponse = querier
            .query_wasm_smart(
                &poap_config.cw721_contract,
                &Cw721QueryMsg::<String>::Minter {},
            )
            .unwrap();

        // The cw721 minter should be the poap contract address.
        assert_eq!(poap_contract_addr.to_string(), cw721_minter_response.minter)
    }

    #[test]
    fn mint_with_permission_properly() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Mint should work since the event is started and the user is allowed to mint
        let msg = ExecuteMsg::Mint {};
        app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let querier = app.wrap();
        let response: QueryMintedAmountResponse = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::MintedAmount {
                    user: USER.to_string(),
                },
            )
            .unwrap();

        assert_eq!(Addr::unchecked(USER), response.user);
        assert_eq!(1, response.amount);

        let config: QueryConfigResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        let querier = app.wrap();
        let response: TokensResponse = querier
            .query_wasm_smart(
                config.cw721_contract.as_str(),
                &Cw721QueryMsg::<Empty>::Tokens {
                    owner: USER.to_string(),
                    start_after: None,
                    limit: None,
                },
            )
            .unwrap();

        assert_eq!(1, response.tokens.len());

        let minted_nft_info: NftInfoResponse<Metadata> = querier
            .query_wasm_smart(
                config.cw721_contract.as_str(),
                &Cw721QueryMsg::<Empty>::NftInfo {
                    token_id: "1".to_string(),
                },
            )
            .unwrap();
        assert_eq!(
            Metadata {
                claimer: Addr::unchecked(USER)
            },
            minted_nft_info.extension
        )
    }

    #[test]
    fn mint_to_with_permission_properly() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Mint should work since the event is started and the user is allowed to mint
        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let querier = app.wrap();
        let response: QueryMintedAmountResponse = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::MintedAmount {
                    user: USER.to_string(),
                },
            )
            .unwrap();

        assert_eq!(Addr::unchecked(USER), response.user);
        assert_eq!(1, response.amount);

        let config: QueryConfigResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        let querier = app.wrap();
        let response: TokensResponse = querier
            .query_wasm_smart(
                config.cw721_contract.as_str(),
                &Cw721QueryMsg::<Empty>::Tokens {
                    owner: USER.to_string(),
                    start_after: None,
                    limit: None,
                },
            )
            .unwrap();

        assert_eq!(1, response.tokens.len());

        let minted_nft_info: NftInfoResponse<Metadata> = querier
            .query_wasm_smart(
                config.cw721_contract.as_str(),
                &Cw721QueryMsg::<Empty>::NftInfo {
                    token_id: "1".to_string(),
                },
            )
            .unwrap();
        assert_eq!(
            Metadata {
                claimer: Addr::unchecked(USER)
            },
            minted_nft_info.extension
        )
    }

    #[test]
    fn query_tokens() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Mint should work since the event is started and the user is allowed to mint
        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let querier = app.wrap();
        let response: QueryMintedAmountResponse = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::MintedAmount {
                    user: USER.to_string(),
                },
            )
            .unwrap();

        assert_eq!(Addr::unchecked(USER), response.user);
        assert_eq!(1, response.amount);

        let config: QueryConfigResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        let querier = app.wrap();
        let response: TokensResponse = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::Tokens {
                    owner: USER.to_string(),
                    start_after: None,
                    limit: None,
                },
            )
            .unwrap();
        let cw721_response: TokensResponse = querier
            .query_wasm_smart(
                config.cw721_contract.as_str(),
                &Cw721QueryMsg::<Empty>::Tokens {
                    owner: USER.to_string(),
                    start_after: None,
                    limit: None,
                },
            )
            .unwrap();
        assert_eq!(cw721_response, response);
        assert_eq!(1, response.tokens.len());
    }

    #[test]
    fn query_nft_info() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Mint should work since the event is started and the user is allowed to mint
        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let querier = app.wrap();
        let response: QueryMintedAmountResponse = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::MintedAmount {
                    user: USER.to_string(),
                },
            )
            .unwrap();

        assert_eq!(Addr::unchecked(USER), response.user);
        assert_eq!(1, response.amount);

        let config: QueryConfigResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        let querier = app.wrap();
        let response: TokensResponse = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::Tokens {
                    owner: USER.to_string(),
                    start_after: None,
                    limit: None,
                },
            )
            .unwrap();
        assert_eq!(1, response.tokens.len());
        let cw721_response: AllNftInfoResponse<Metadata> = querier
            .query_wasm_smart(
                config.cw721_contract.as_str(),
                &Cw721QueryMsg::<Empty>::AllNftInfo {
                    token_id: "1".to_string(),
                    include_expired: None,
                },
            )
            .unwrap();
        let response: AllNftInfoResponse<Metadata> = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::AllNftInfo {
                    token_id: "1".to_string(),
                    include_expired: None,
                },
            )
            .unwrap();
        assert_eq!(cw721_response, response);
        assert_eq!(
            AllNftInfoResponse {
                access: OwnerOfResponse {
                    owner: USER.to_string(),
                    approvals: vec![]
                },
                info: NftInfoResponse {
                    token_uri: Some(POAP_URI.to_string()),
                    extension: Metadata {
                        claimer: Addr::unchecked(USER)
                    },
                }
            },
            response
        );
    }
}

use crate::profiles::{
    models_app_links::{AppLinkResult, ApplicationLink, CallData, Data, OracleRequest},
    models_blocks::UserBlock,
    models_chain_links::{ChainConfig, ChainLink, ChainLinkAddr, Proof, Signature},
    models_common::PubKey,
    models_dtag_requests::DtagTransferRequest,
    models_profile::{Account, Pictures, Profile},
    models_relationships::Relationship,
};
use cosmwasm_std::{Addr};

/**
This file contains some useful mocks of the Desmos x/profiles modules types ready made to be used
in any test
**/

pub struct MockProfilesQueries {}

impl MockProfilesQueries {
    pub fn get_mock_profile() -> Profile {
        Profile {
            account: Account {
                proto_type: "/cosmos.auth.v1beta1.BaseAccount".to_string(),
                address: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: "ArlRm0a5fFTHFfKha1LpDd+g3kZlyRBBF4R8PSM8Zo4Y".to_string(),
                },
                account_number: "0".to_string(),
                sequence: "15".to_string(),
            },
            dtag: "goldrake".to_string(),
            nickname: "Goldrake".to_string(),
            bio: "This is Goldrake".to_string(),
            pictures: Pictures {
                profile: "".to_string(),
                cover: "".to_string(),
            },
            creation_date: "2022-02-21T13:18:27.257641Z".to_string(),
        }
    }

    pub fn get_mock_dtag_transfer_request() -> DtagTransferRequest {
        DtagTransferRequest {
            dtag_to_trade: "goldrake".to_string(),
            sender: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            receiver: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
        }
    }

    pub fn get_mock_relationship() -> Relationship {
        Relationship {
            creator: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            recipient: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
            subspace_id: "1".to_string(),
        }
    }

    pub fn get_mock_user_block() -> UserBlock {
        UserBlock {
            blocker: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            blocked: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
            reason: "test".to_string(),
            subspace_id: "1".to_string(),
        }
    }

    pub fn get_mock_chain_link() -> ChainLink {
        ChainLink{
            user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            address: ChainLinkAddr {
                proto_type: "/desmos.profiles.v1beta1.Bech32Address".to_string(),
                value: "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string(),
                prefix: "cosmos".to_string()
            },
            proof: Proof {
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: "AyRUhKXAY6zOCjjFkPN78Q29sBKHjUx4VSZQ4HXh66IM".to_string()
                },
                signature: Signature {
                    proto_type: "/desmos.profiles.v1beta1.SingleSignatureData".to_string(),
                    signature: "C7xppu4C4S3dgeC9TVqhyGN1hbMnMbnmWgXQI2WE8t0oHIHhDTqXyZgzhNNYiBO7ulno3G8EXO3Ep5KMFngyFg".to_string()
                },
                plain_text: "636f736d6f733138786e6d6c7a71727172367a74353236706e637a786536357a6b33663478676d6e6470786e32".to_string()
            },
            chain_config: ChainConfig { name: "cosmos".to_string() },
            creation_time: "2022-02-21T13:18:57.800827Z".to_string()
        }
    }

    pub fn get_mock_application_link() -> ApplicationLink {
        ApplicationLink {
            user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            data: Data {
                application: "twitter".to_string(),
                username: "goldrake".to_string(),
            },
            state: "APPLICATION_LINK_STATE_VERIFICATION_SUCCESS".to_string(),
            oracle_request: OracleRequest {
                id: "537807".to_string(),
                oracle_script_id: "32".to_string(),
                call_data: CallData {
                    application: "twitter".to_string(),
                    call_data: "7b22757365726e616d65223a224c756361675f5f2335323337227d".to_string(),
                },
                client_id: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc-twitter-goldrake"
                    .to_string(),
            },
            result: AppLinkResult::Success {
                value: "4c756361675f5f2345423337".to_string(),
                signature: "9690d734171298eb4cc9636c36d8507535264c1fdb136c9095a6a50c41ccffa"
                    .to_string(),
            },
            creation_time: "2022-02-21T13:18:57.800827Z".to_string(),
        }
    }
}

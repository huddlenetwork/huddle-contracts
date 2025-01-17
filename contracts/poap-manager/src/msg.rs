use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};

use crate::error::ContractError;
use poap::msg::InstantiateMsg as POAPInstantiateMsg;

#[cw_serde]
#[schemars(rename = "PoapManagerInstantiateMsg", title = "InstantiateMsg")]
pub struct InstantiateMsg {
    /// Address of who will have the right to administer the contract.
    pub admin: String,
    /// Id of the POAP contract to be initialized along with this contract.
    pub poap_code_id: Uint64,
    /// Initialization message of the POAP contract.
    pub poap_instantiate_msg: POAPInstantiateMsg,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.poap_code_id == Uint64::zero() {
            return Err(ContractError::InvalidPOAPCodeID {});
        }
        Ok(())
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Allows users to claim a POAP token.
    Claim {},
    /// Allows the contract's admin to mint a POAP for a specific recipient.
    MintTo { recipient: String },
    /// Allows the contract's admin to transfer the admin rights to another user.
    UpdateAdmin { new_admin: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns a ConfigResponse containing the configuration info of the Manager contract
    #[returns(QueryConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct QueryConfigResponse {
    /// Address of the contract administrator.
    pub admin: Addr,
    /// Id of the POAP contract that this contract has initialized.
    pub poap_code_id: u64,
    /// Address of the POAP contract
    pub poap_contract_address: Addr,
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Timestamp;
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use poap::msg::EventInfo;

    #[test]
    fn instantiate_msg_with_invalid_poap_id_error() {
        let msg = InstantiateMsg {
            admin: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc".into(),
            poap_code_id: 0u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: "test".into(),
                minter: "test".into(),
                cw721_code_id: 2u64.into(),
                cw721_instantiate_msg: Cw721InstantiateMsg {
                    minter: "".into(),
                    name: "test".into(),
                    symbol: "test".into(),
                },
                event_info: EventInfo {
                    creator: "creator".to_string(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    poap_uri: "ipfs://popap-uri".to_string(),
                },
            },
        };
        let result = msg.validate();
        assert_eq!(result.unwrap_err(), ContractError::InvalidPOAPCodeID {},)
    }

    #[test]
    fn proper_instantiate_msg_no_error() {
        let msg = InstantiateMsg {
            admin: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc".into(),
            poap_code_id: 1u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: "test".into(),
                minter: "test".into(),
                cw721_code_id: 2u64.into(),
                cw721_instantiate_msg: Cw721InstantiateMsg {
                    minter: "".into(),
                    name: "test".into(),
                    symbol: "test".into(),
                },
                event_info: EventInfo {
                    creator: "creator".to_string(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    poap_uri: "ipfs://popap-uri".to_string(),
                },
            },
        };
        msg.validate().unwrap();
    }
}

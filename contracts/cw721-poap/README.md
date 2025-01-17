# CW721 POAP contract

Contract that defines the cw721-base contract having custom POAP `Metadata`, which is used by POAP contract.
To easily interact with the contract you can use the `cw721-poap` script available [here](https://github.com/desmos-labs/contract-utils/tree/main/utils), 
otherwise you can take a look at the supported messages in the following sections.

## Instantiate Message
Allows to initialize the contract. This message has the following parameters:
* `name`: Name of the NFT contract;
* `symbol`: Symbol of the NFT contract;
* `minter`: Address who is the only one to be able to create new NFTs.

Here an example message to instantiate a contract:
```json
{
    "name": "test_name",
    "symbol": "test",
    "minter": "desmos1......"
}
```

## Execute Messages

### TransferNft
Allows to move a token to another account without triggering actions. This message has the following parameters:
* `recipient`: Address where the token transfer to;
* `token_id`: Id of the token which would be transferred.

Here an example message to transfer nft:
```json
{
    "transfer_nft": {
        "recipient": "desmos1......",
        "token_id": "1"
    }
}
```

### SendNft
Allows to move a token to another contract then trigger an action. This message has the following parameters:
* `contract`: Contract address where the token transfer to;
* `token_id`: Id of the token which would be transferred;
* `msg`: Base64 encoded message to trigger on the receiver contract.

Here an example message to send nft having a trigger message:
```json
{
    "send_nft": {
        "contract": "desmos1......",
        "token_id": "1",
        "msg": "eyJleGVjdXRlX2V4YW1wbGUiOnt9fQ=="
    }
}
```
**Note** the msg is base64-encoded of `{"execute_example":{}}`, which is the sample execution message on the target contract. 

### Approve
Allows a user to transfer/send the token from the owner's account. This message has the following parameters:
* `spender`: Address who would be assigned as an access of the token;
* `token_id`: Id of the target token;
* `expires`: Expiration time/height of this allowance, if it is set as `null` then it has no time/height limit.

An example message to approve a user to have the sending/transferring access to a token with an expiration height:
```json
{
    "approve": {
        "spender": "desmos1......",
        "token_id": "1",
        "expires": {
            "at_height": 1000
        }
    }
}
```

Here an example message to approve a token to an operator with an expiration time:
```json
{
    "approve": {
        "spender": "desmos1......",
        "token_id": "1",
        "expires": {
            "at_time": "2022-01-01T00:00:00Z"
        }
    }
}
```

Here an example message to approve a token to an operator without any expiration:
```json
{
    "approve": {
        "spender": "desmos1......",
        "token_id": "1",
        "expires": null
    }
}
```

### Revoke
Allows to remove a previously granted approval. This message has the following parameters:
* `spender`: Address who would be revoked the permission of the given token;
* `token_id`: Id of the target token.

Here an example meesage to revoke an operator to a token:
```json
{
    "revoke": {
        "spender": "desmos1......",
        "token_id": "1"
    }
}
```

### ApproveAll
Allows to give all the tokens transferring/sendind tokens approval to an operator from the owner's account. This message has the following parameters:
* `operator`: Address who is assigned to have all the tokens approvals in the owner's account;
* `expires`: Expiration time/height of this allowance, if it is set as `null` then it has no time/height limit.

Here an example meesage to approve an operator all the tokens with an expiration height:
```json
{
    "approve_all": {
        "spender": "desmos1......",
        "expires": {
            "at_height": 1000
        }
    }
}
```

Here an example meesage to approve an operator all the tokens with an expiration time:
```json
{
    "approve_all": {
        "spender": "desmos1......",
        "expires": {
            "at_time": "2022-01-01T00:00:00Z"
        }
    }
}
```

Here an example meesage to approve an operator all the tokens without any expiration:
```json
{
    "approve_all": {
        "spender": "desmos1......",
        "expires": null
    }
}
```

### RevokeAll
Allows to remove a previously granted approval all permission. This message has the following parameters:
* `operator`: Address who would be revoked operator permissions of all the tokens from the owner's account.

Here an example meesage to revoke operator permissions to all the tokens from the owner's account:
```json
{
    "revoke": {
        "spender": "desmos1......",
    }
}
```

### Mint
Allows the minter to mint a new NFT to a user. This message has the following parameters:
* `token_id`: unique id of the NFT;
* `owner`: the owner of the newly minted NFT;
* `token_uri`: universal resource identifier for this NFT;
* `extension`: the `POAP metadata` which includes claimer of this NFT.

Here an example meesage to mint new NFT:
```json
{
    "mint": {
        "token_id": "1",
        "owner": "desmos1......",
        "token_uri": "ipfs://token.erc721.metadata",
        "extension": {
            "claimer": "desmos1......"
        }
    }
}
```

### Burn
Allows to burn an NFT the sender has access to. This message has the following parameters:
* `token_id`: Id of the token that would be burned.

Here an example meesage to burn an NFT:
```json
{
    "burn": {
        "token_id": "1"
    }
}
```

## Query Messages

### OwnerOf
Returns the owner of the given token, error if token does not exist. This message has the following parameters:
* `token_id`: Id of the target token;
* `include_expired`: Trigger to filter out expired approvals, unset or false will exclude expired approvals.

Here an example meesage to query the owner of the given token:
```json
{
    "owner_of": {
        "token_id": "1",
        "includ_expired": true,
    }
}
```

Response:
```json
{
    "owner": "desmos1......",
    "approvals": [
        {
            "spender": "desmos1......",
            "expiration": {
                "at_height": 1000
            }
        }, 
        {
            "spender": "desmos1......",
            "expiration": {
                "at_time": "2022-01-01T00:00:00Z"
            }
        },
        {
            "spender": "desmos1......",
            "expiration": {
                "never": {}
            }
        },
    ]
}
```

### Approval
Returns an access owned by the given spender to the given token. This message has the following parameters:
* `token_id`: Id of the target token;
* `spender`: Address who has the sending/transferring access to the given token;
* `include_expired`: Trigger to filter out expired approvals, unset or false will exclude expired approvals.

Here an example meesage to query the approval of the given token by a spender:
```json
{
    "approval": {
        "token_id": "1",
        "spender": "desmos1......",
        "includ_expired": true,
    }
}
```

Response:
```json
{
    "approval": {
        "spender": "desmos1......",
        "expiration": {
            "at_height": 1000
        }
    } 
}
```

### Approvals
Returns approvals that a token has. This message has the following parameters:
* `token_id`: Id of the target token;
* `include_expired`: Trigger to filter out expired approvals, unset or false will exclude expired approvals.

Here an example meesage to query the approvals of the given token:
```json
{
    "approvals": {
        "token_id": "1",
        "includ_expired": true,
    }
}
```

Response:
```json
{
    "approvals": [
        {
            "spender": "desmos1......",
            "expiration": {
                "at_height": 1000
            }
        }, 
        {
            "spender": "desmos1......",
            "expiration": {
                "at_time": "2022-01-01T00:00:00Z"
            }
        },
        {
            "spender": "desmos1......",
            "expiration": {
                "never": {}
            }
        },
    ]
}
```

### AllOperators
Lists all operators that can access all of the owner's tokens. This message has the following parameters:
* `owner`: Address of the owner to be queried.
* `include_expired`: Trigger to filter out expired approvals, unset or false will exclude expired approvals;
* `start_after`: Position in address where tokens start after;
* `limit`: Limitation to list the number of operators, if unset would be 10 and the maximum is 100.

Here an example meesage to query the operators of the given owner:
```json
{
    "all_operators": {
        "owner": "desmos1......",
        "includ_expired": true,
        "start_after": "desmos1......",
        "limit": 10
    }
}
```

Response:
```json
{
    "operators": [
        {
            "spender": "desmos1......",
            "expiration": {
                "at_height": 1000
            }
        }, 
        {
            "spender": "desmos1......",
            "expiration": {
                "at_time": "2022-01-01T00:00:00Z"
            }
        },
        {
            "spender": "desmos1......",
            "expiration": {
                "never": {}
            }
        },
    ]
}
```

### NumTokens
Returns total number of tokens issued.

Here an example meesage to query total number of tokens:
```json
{
    "num_tokens": {}
}
```

Response:
```json
{
    "count": 1000
}
```

### ContractInfo
Returns top-level metadata about the contract.

Here an example meesage to query the contract info of the contract:
```json
{
    "contract_info": {}
}
```

Response:
```json
{
    "name": "test_name",
    "symbol": "test_symbol"
}
```

### NftInfo
Returns metadata about one particular token. This message has the following parameters:
* `token_id`: Id of the target token.

Here an example meesage to query the info of the given token:
```json
{
    "nft_info": {
        "token_id": "1"
    }
}
```

Response:
```json
{
    "token_uri": "ipfs://token.erc721.metadata",
    "extension": {
        "claimer": "desmos1......"
    }
}
```

### AllNftInfo
Returns the result of both `NftInfo` and `OwnerOf`. This message has the following parameters:
* `token_id`: Id of the target token.
* `include_expired`: Trigger to filter out expired approvals, unset or false will exclude expired approvals.

Here an example meesage to query all the info of the given token:
```json
{
    "all_nft_info": {
        "token_id": "1",
        "include_expired": true
    }
}
```

Response:
```json
{
    "access": {
        "owner": "desmos1......",
        "approvals": [
            {
                "spender": "desmos1......",
                "expiration": {
                    "at_height": 1000
                }
            }, 
            {
                "spender": "desmos1......",
                "expiration": {
                    "at_time": "2022-01-01T00:00:00Z"
                }
            },
            {
                "spender": "desmos1......",
                "expiration": {
                    "never": {}
                }
            },
        ],
    },
    "info": {
        "token_uri": "ipfs://token.erc721.metadata",
        "extension": {
            "claimer": "desmos1......"
        }
    }
}
```

### Tokens
Returns all tokens owned by the given address. This message has the following parameters:
* `owner`: Target address owned tokens to be queried;
* `start_after`: Position in token id where tokens start after;
* `limit`: Limitation to list the number of tokens, if unset would be 10 and the maximum is 100.

Here an example meesage to query all the tokens owned by the given address:
```json
{
    "tokens": {
        "owner": "desmos1......",
        "start_after": "1",
        "limit": 3
    }
}
```

Response:
```json
{
    "tokens": ["2", "3", "4"]
}
```

### AllTokens
Lists all token_ids in the contract. This message has the following parameters:
* `start_after`: Position in token id where tokens start after;
* `limit`: Limitation to list the number of tokens, if unset would be 10 and the maximum is 100.

Here an example meesage to query all the tokens in the contract:
```json
{
    "tokens": {
        "start_after": "1",
        "limit": 3
    }
}
```

Response:
```json
{
    "tokens": ["2", "3", "4"]
}
```

### Minter
Returns the minter who is the one having access to mint NFT.

Here an example meesage to query the minter of the contract:
```json
{
    "minter": {}
}
```

Response:
```json
{
    "minter": "desmos1......"
}
```
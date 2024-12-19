/// Generate test vectors as binary files for use by tests.
#[cfg(test)]
pub mod tests {
    use crate::tx_intent_test_data::tests::*;
    use std::io::Write;

    struct Blob {
        pub name: &'static str,
        pub data: &'static [u8],
    }

    const BLOBS: &[Blob] = &[
        Blob {
            name: "access_rule",
            data: &TX_ACCESS_RULE,
        },
        Blob {
            name: "call_function",
            data: &TX_CALL_FUNCTION,
        },
        Blob {
            name: "call_method",
            data: &TX_CALL_METHOD,
        },
        Blob {
            name: "create_access_controller",
            data: &TX_CREATE_ACCESS_CONTROLLER,
        },
        Blob {
            name: "create_account",
            data: &TX_CREATE_ACCOUNT,
        },
        Blob {
            name: "create_fungible_resource_with_initial_supply",
            data: &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
        },
        Blob {
            name: "create_fungible_resource_with_no_initial_supply",
            data: &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
        },
        Blob {
            name: "create_identity",
            data: &TX_CREATE_IDENTITY,
        },
        Blob {
            name: "create_non_fungible_resource_with_no_initial_supply",
            data: &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
        },
        Blob {
            name: "metadata",
            data: &TX_METADATA,
        },
        Blob {
            name: "mint_fungible",
            data: &TX_MINT_FUNGIBLE,
        },
        Blob {
            name: "mint_non_fungible",
            data: &TX_MINT_NON_FUNGIBLE,
        },
        Blob {
            name: "publish_package",
            data: &TX_PUBLISH_PACKAGE,
        },
        Blob {
            name: "resource_auth_zone",
            data: &TX_RESOURCE_AUTH_ZONE,
        },
        Blob {
            name: "resource_recall",
            data: &TX_RESOURCE_RECALL,
        },
        Blob {
            name: "resource_recall_nonfungibles",
            data: &TX_RESOURCE_RECALL_NONFUNGIBLES,
        },
        Blob {
            name: "resource_worktop",
            data: &TX_RESOURCE_WORKTOP,
        },
        Blob {
            name: "royalty",
            data: &TX_ROYALTY,
        },
        Blob {
            name: "values",
            data: &TX_VALUES,
        },
        Blob {
            name: "simple_transfer",
            data: &TX_SIMPLE_TRANSFER,
        },
        Blob {
            name: "simple_invalid_transfer",
            data: &TX_SIMPLE_INVALID_TRANSFER,
        },
        Blob {
            name: "simple_transfer_new_format",
            data: &TX_SIMPLE_TRANSFER_NEW_FORMAT,
        },
        Blob {
            name: "simple_transfer_nft",
            data: &TX_SIMPLE_TRANSFER_NFT,
        },
        Blob {
            name: "simple_transfer_nft_new_format",
            data: &TX_SIMPLE_TRANSFER_NFT_NEW_FORMAT,
        },
        Blob {
            name: "simple_transfer_nft_by_id",
            data: &TX_SIMPLE_TRANSFER_NFT_BY_ID,
        },
        Blob {
            name: "simple_transfer_nft_by_id_new_format",
            data: &TX_SIMPLE_TRANSFER_NFT_BY_ID_NEW_FORMAT,
        },
        Blob {
            name: "simple_transfer_with_multiple_locked_fees",
            data: &TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES,
        },
        Blob {
            name: "address_allocation",
            data: &TX_ADDRESS_ALLOCATION,
        },
        Blob {
            name: "create_non_fungible_resource_with_initial_supply",
            data: &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
        },
        Blob {
            name: "create_validator",
            data: &TX_CREATE_VALIDATOR,
        },
    ];

    #[test]
    pub fn generate_binaries() {
        for blob in BLOBS {
            let mut file =
                std::fs::File::create(format!("../test/data/{}.txn", blob.name)).unwrap();
            file.write_all(blob.data).unwrap();
        }
    }
}

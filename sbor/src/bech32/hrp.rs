use crate::bech32::network::NetworkId;

pub fn hrp_suffix(net_id: NetworkId) -> &'static [u8] {
    match net_id {
        NetworkId::OlympiaMainNet => b"rdx",
        NetworkId::MainNet => b"rdx",
        NetworkId::StokeNet => b"tdx_2_",
        NetworkId::AdapaNet => b"tdx_a_",
        NetworkId::NebuNet => b"tdx_b_",
        NetworkId::KisharNet => b"tdx_c_",
        NetworkId::AnsharNet => b"tdx_d_",
        NetworkId::GilgaNet => b"tdx_20_",
        NetworkId::EnkiNet => b"tdx_21_",
        NetworkId::HammuNet => b"tdx_22_",
        NetworkId::NergalNet => b"tdx_23_",
        NetworkId::MarduNet => b"tdx_24_",
        NetworkId::LocalNet => b"loc",
        NetworkId::IntegrationTestNet => b"test",
        NetworkId::Simulator => b"sim",
    }
}

// From radix-engine-common/src/types/entity_type.rs
const GLOBAL_PACKAGE: u8 = 0b00001101;
const GLOBAL_CONSENSUS_MANAGER: u8 = 0b10000110;
const GLOBAL_VALIDATOR: u8 = 0b10000011;
const GLOBAL_TRANSACTION_TRACKER: u8 = 0b10000010;
const GLOBAL_GENERIC_COMPONENT: u8 = 0b11000000;
const GLOBAL_ACCOUNT: u8 = 0b11000001;
const GLOBAL_IDENTITY: u8 = 0b11000010;
const GLOBAL_ACCESS_CONTROLLER: u8 = 0b11000011;
const GLOBAL_ONE_RESOURCE_POOL: u8 = 0b11000100;
const GLOBAL_TWO_RESOURCE_POOL: u8 = 0b11000101;
const GLOBAL_MULTI_RESOURCE_POOL: u8 = 0b11000110;
const GLOBAL_VIRTUAL_SECP256K1_ACCOUNT: u8 = 0b11010001;
const GLOBAL_VIRTUAL_SECP256K1_IDENTITY: u8 = 0b11010010;
const GLOBAL_VIRTUAL_ED25519_ACCOUNT: u8 = 0b01010001;
const GLOBAL_VIRTUAL_ED25519_IDENTITY: u8 = 0b01010010;
const GLOBAL_FUNGIBLE_RESOURCE_MANAGER: u8 = 0b01011101;
const INTERNAL_FUNGIBLE_VAULT: u8 = 0b01011000;
const GLOBAL_NON_FUNGIBLE_RESOURCE_MANAGER: u8 = 0b10011010;
const INTERNAL_NON_FUNGIBLE_VAULT: u8 = 0b10011000;
const INTERNAL_GENERIC_COMPONENT: u8 = 0b11111000;
const INTERNAL_ACCOUNT: u8 = 0b11111001;
const INTERNAL_KEY_VALUE_STORE: u8 = 0b10110000;

// From radix-engine-common/src/address/hrpset.rs
struct HrpSet {
    /* Entities */
    pub package: &'static [u8],
    pub resource: &'static [u8],
    pub component: &'static [u8],
    pub account: &'static [u8],
    pub identity: &'static [u8],
    pub consensus_manager: &'static [u8],
    pub validator: &'static [u8],
    pub access_controller: &'static [u8],
    pub pool: &'static [u8],
    pub transaction_tracker: &'static [u8],
    pub internal_vault: &'static [u8],
    pub internal_account: &'static [u8],
    pub internal_component: &'static [u8],
    pub internal_key_value_store: &'static [u8],
}

const HRP_SET: HrpSet = HrpSet {
    /* Entities */
    package: b"package_",
    resource: b"resource_",
    component: b"component_",
    account: b"account_",
    identity: b"identity_",
    consensus_manager: b"consensusmanager_",
    validator: b"validator_",
    access_controller: b"accesscontroller_",
    pool: b"pool_",
    transaction_tracker: b"transactiontracker_",
    internal_vault: b"internal_vault_",
    internal_account: b"internal_account_",
    internal_component: b"internal_component_",
    internal_key_value_store: b"internal_keyvaluestore_",
};

pub fn hrp_prefix(hrp_type: u8) -> Option<&'static [u8]> {
    match hrp_type {
        GLOBAL_PACKAGE => Some(HRP_SET.package),
        GLOBAL_FUNGIBLE_RESOURCE_MANAGER => Some(HRP_SET.resource),
        GLOBAL_NON_FUNGIBLE_RESOURCE_MANAGER => Some(HRP_SET.resource),
        GLOBAL_CONSENSUS_MANAGER => Some(HRP_SET.consensus_manager),
        GLOBAL_VALIDATOR => Some(HRP_SET.validator),
        GLOBAL_ACCESS_CONTROLLER => Some(HRP_SET.access_controller),
        GLOBAL_ACCOUNT => Some(HRP_SET.account),
        GLOBAL_IDENTITY => Some(HRP_SET.identity),
        GLOBAL_GENERIC_COMPONENT => Some(HRP_SET.component),
        GLOBAL_VIRTUAL_SECP256K1_ACCOUNT => Some(HRP_SET.account),
        GLOBAL_VIRTUAL_ED25519_ACCOUNT => Some(HRP_SET.account),
        GLOBAL_VIRTUAL_SECP256K1_IDENTITY => Some(HRP_SET.identity),
        GLOBAL_VIRTUAL_ED25519_IDENTITY => Some(HRP_SET.identity),
        INTERNAL_FUNGIBLE_VAULT => Some(HRP_SET.internal_vault),
        INTERNAL_NON_FUNGIBLE_VAULT => Some(HRP_SET.internal_vault),
        INTERNAL_ACCOUNT => Some(HRP_SET.internal_account),
        INTERNAL_GENERIC_COMPONENT => Some(HRP_SET.internal_component),
        INTERNAL_KEY_VALUE_STORE => Some(HRP_SET.internal_key_value_store),
        GLOBAL_ONE_RESOURCE_POOL | GLOBAL_TWO_RESOURCE_POOL | GLOBAL_MULTI_RESOURCE_POOL => {
            Some(HRP_SET.pool)
        }
        GLOBAL_TRANSACTION_TRACKER => Some(HRP_SET.transaction_tracker),
        _ => None,
    }
}

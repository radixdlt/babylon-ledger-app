use crate::bech32::network::NetworkId;

pub fn hrp_suffix(net_id: NetworkId) -> &'static str {
    match net_id {
        NetworkId::OlympiaMainNet => "rdx",
        NetworkId::MainNet => "rdx",
        NetworkId::StokeNet => "tdx_2_",
        NetworkId::AdapaNet => "tdx_a_",
        NetworkId::NebuNet => "tdx_b_",
        NetworkId::KisharNet => "tdx_c_",
        NetworkId::AnsharNet => "tdx_d_",
        NetworkId::GilgaNet => "tdx_20_",
        NetworkId::EnkiNet => "tdx_21_",
        NetworkId::HammuNet => "tdx_22_",
        NetworkId::NergalNet => "tdx_23_",
        NetworkId::MarduNet => "tdx_24_",
        NetworkId::LocalNet => "loc",
        NetworkId::IntegrationTestNet => "test",
        NetworkId::Simulator => "sim",
    }
}

pub fn hrp_prefix(hrp_type: u8) -> Option<&'static str> {
    match hrp_type {
        0b00001101 => Some("package_"),
        0b01011101 | 0b10011010 => Some("resource_"),
        0b10000110 => Some("consensusmanager_"),
        0b10000010 => Some("validator_"),
        0b11000011 => Some("accesscontroller_"),
        0b11000001 | 0b11010001 | 0b01010001 => Some("account_"),
        0b11000010 | 0b11010010 | 0b01010010 => Some("identity_"),
        0b11000000 => Some("component_"),
        0b01011000 | 0b10011000 => Some("internal_vault_"),
        0b11111001 => Some("internal_account_"),
        0b11111000 => Some("internal_component_"),
        0b10110000 => Some("internal_keyvaluestore_"),
        0b11000100 | 0b11000101 | 0b11000110 => Some("pool_"),
        _ => None,
    }
}

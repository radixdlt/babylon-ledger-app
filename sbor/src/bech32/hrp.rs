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
        0x00 => Some("package_"),
        0x01 | 0x02 => Some("resource_"),
        0x03 => Some("component_"),
        0x04 | 0x08 | 0x09 => Some("account_"),
        0x0A | 0x0B | 0x0C => Some("identity_"),
        0x05 => Some("epochmanager_"),
        0x06 => Some("validator_"),
        0x07 => Some("clock_"),
        0x0D => Some("accesscontroller_"),

        _ => None,
    }
}

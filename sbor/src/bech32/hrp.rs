use crate::bech32::network::NetworkId;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HrpType {
    Package,
    FungibleResource,
    NonFungibleResource,
    Component,
    Autodetect,
}

pub fn hrp_suffix(net_id: NetworkId) -> &'static str {
    match net_id {
        NetworkId::MainNet => "rdx",
        NetworkId::StokeNet => "tdx_2_",
        NetworkId::AdapaNet => "tdx_a_",
        NetworkId::NebuNet => "tdx_b_",
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

pub fn hrp_prefix(entity_id: HrpType, discriminator: u8) -> Option<&'static str> {
    match entity_id {
        HrpType::Autodetect => None,
        HrpType::Package => Some("package_"),
        HrpType::FungibleResource | HrpType::NonFungibleResource => Some("resource_"),
        HrpType::Component => match discriminator {
            // NOTE: this part depends on Scrypto entity ID's
            0x03 => Some("component_"), // Normal
            0x05 => Some("epochmanager_"),      // EpochManager
            0x06 => Some("validator_"),         // Validator
            0x07 => Some("clock_"),             // Clock
            0x0d => Some("accesscontroller_"), // AccessController
            0x04 | 0x08 | 0x09 => Some("account_"), // Account, EcdsaSecp256k1VirtualAccount, EddsaEd25519VirtualAccount
            0x0a | 0x0b | 0x0c => Some("identity_"), // Identity, EcdsaSecp256k1VirtualIdentity, EddsaEd25519VirtualIdentity
            _ => None,
        },
    }
}

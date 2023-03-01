use crate::bech32::network::NetworkId;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum HrpType {
    Package,
    Resource,
    Component,
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
        NetworkId::LocalSimulator => "sim",
    }
}

pub fn hrp_prefix(entity_id: HrpType, discriminator: u8) -> Option<&'static str> {
    match entity_id {
        HrpType::Package => Some("package_"),
        HrpType::Resource => Some("resource_"),
        HrpType::Component => match discriminator {
            // NOTE: this part depends on Scrypto ComponentAddress enum
            0 => Some("component_"), // Normal

            1 | 6 | 7 => Some("account_"), // Account, EcdsaSecp256k1VirtualAccount, EddsaEd25519VirtualAccount
            2 | 8 | 9 => Some("identity_"), // Identity, EcdsaSecp256k1VirtualIdentity, EddsaEd25519VirtualIdentity

            3 => Some("clock_"),             // Clock
            4 => Some("epochmanager_"),      // EpochManager
            5 => Some("validator_"),         // Validator
            10 => Some("accesscontroller_"), // AccessController
            _ => None,
        },
    }
}

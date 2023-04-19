use crate::bech32::network::NetworkId;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HrpType {
    Package,
    Resource,
    Component,
    Account,
    Identity,
    EpochManager,
    Clock,
    Validator,
    AccessController,
    InternalVault,
    InternalAccount,
    InternalComponent,
    InternalKeyValueStore,
}

pub fn hrp_suffix(net_id: NetworkId) -> &'static str {
    match net_id {
        NetworkId::OlympiaMainNet => "rdx",
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

pub fn hrp_prefix(hrp_type: HrpType) -> &'static str {
    match hrp_type {
        HrpType::Package => "package_",
        HrpType::Resource => "resource_",
        HrpType::Component => "component_",
        HrpType::Account => "account_",
        HrpType::Identity => "identity_",
        HrpType::EpochManager => "epochmanager_",
        HrpType::Clock => "clock_",
        HrpType::Validator => "validator_",
        HrpType::AccessController => "accesscontroller_",
        HrpType::InternalVault => "internal_vault_",
        HrpType::InternalAccount => "internal_account_",
        HrpType::InternalComponent => "internal_component_",
        HrpType::InternalKeyValueStore => "internal_keyvaluestore_",
    }
}

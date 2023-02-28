#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NetworkId {
    MainNet = 1,              //, "mainnet", "rdx"),
    StokeNet = 2,             //, "stokenet", "tdx_2_"),
    AdapaNet = 10,            //, "adapanet", "tdx_a_"),
    NebuNet = 11,             //, "nebunet", "tdx_b_"),
    GilgaNet = 32,            //, "gilganet", "tdx_20_"),
    EnkiNet = 33,             //, "enkinet", "tdx_21_"),
    HammuNet = 34,            //, "hammunet", "tdx_22_"),
    NergalNet = 35,           //, "nergalnet", "tdx_23_"),
    MarduNet = 36,            //, "mardunet", "tdx_24_"),
    LocalNet = 240,           //, "localnet", "loc"),
    IntegrationTestNet = 241, //, "inttestnet", "test"),
    LocalSimulator = 242,     //, "simulator", "sim");
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ResourceId {
    Package,
    Resource,
    Component,
    Account,
    Identity,
    EpochManager,
    Clock,
    Validator,
    AccessController,
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

pub fn hrp_prefix(resource_id: ResourceId) -> &'static str {
    match resource_id {
        ResourceId::Package => "package_",
        ResourceId::Resource => "resource_",
        ResourceId::Component => "component_",
        ResourceId::Account => "account_",
        ResourceId::Identity => "identity_",
        ResourceId::EpochManager => "epochmanager_",
        ResourceId::Clock => "clock_",
        ResourceId::Validator => "validator_",
        ResourceId::AccessController => "accesscontroller_",
    }
}

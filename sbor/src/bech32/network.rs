#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum NetworkId {
    OlympiaMainNet = 0,       //, "olympiamainnet", "rdx"),
    MainNet = 1,              //, "mainnet", "rdx"),
    StokeNet = 2,             //, "stokenet", "tdx_2_"),
    AdapaNet = 10,            //, "adapanet", "tdx_a_"),
    NebuNet = 11,             //, "nebunet", "tdx_b_"),
    KisharNet = 12,           //, "kisharnet", "tdx_c_"),
    AnsharNet = 13,           //, "ansharnet", "tdx_d_"),
    GilgaNet = 32,            //, "gilganet", "tdx_20_"),
    EnkiNet = 33,             //, "enkinet", "tdx_21_"),
    HammuNet = 34,            //, "hammunet", "tdx_22_"),
    NergalNet = 35,           //, "nergalnet", "tdx_23_"),
    MarduNet = 36,            //, "mardunet", "tdx_24_"),
    LocalNet = 240,           //, "localnet", "loc"),
    IntegrationTestNet = 241, //, "inttestnet", "test"),
    Simulator = 242,          //, "simulator", "sim");
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum NetworkIdErrors {
    UnknownNetworkId,
}

// Note that this implementation deliberately does not support obtaining OlympiaMainNet from a u32
impl TryFrom<u32> for NetworkId {
    type Error = NetworkIdErrors;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NetworkId::MainNet),
            2 => Ok(NetworkId::StokeNet),
            10 => Ok(NetworkId::AdapaNet),
            11 => Ok(NetworkId::NebuNet),
            12 => Ok(NetworkId::KisharNet),
            13 => Ok(NetworkId::AnsharNet),
            32 => Ok(NetworkId::GilgaNet),
            33 => Ok(NetworkId::EnkiNet),
            34 => Ok(NetworkId::HammuNet),
            35 => Ok(NetworkId::NergalNet),
            36 => Ok(NetworkId::MarduNet),
            240 => Ok(NetworkId::LocalNet),
            241 => Ok(NetworkId::IntegrationTestNet),
            242 => Ok(NetworkId::Simulator),
            _ => Err(NetworkIdErrors::UnknownNetworkId),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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
    Simulator = 242,          //, "simulator", "sim");
}

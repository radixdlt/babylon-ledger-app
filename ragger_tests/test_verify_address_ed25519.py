from ragger.bip import pack_derivation_path

CLA = 0xAA
INS = 0x81


# --------------------------------------------------------------------------------------------
# Check single test vector
# --------------------------------------------------------------------------------------------

def call_and_check(backend, path, expected_pub_key):
    print("Checking ", path, " - ", expected_pub_key)
    response = backend.exchange(cla=CLA, ins=INS, data=pack_derivation_path(path))
    print("Response: ", response.data)
    pk = response.decode('utf-8')
    assert pk == expected_pub_key, "Invalid address\nExpected: " + expected_pub_key + "\nReceived: " + pk
    print(path, " - Success")


test_vectors = [
    ("m/44'/1022'/1'/525'/1460'/0'", "account_rdx12939q7jvjc0q9vvqtc87uv6eu334yagtak7k9udekafy66gpvu222n"),
    ("m/44'/1022'/2'/525'/1460'/1'", "account_tdx_2_12y40zqyxrt27h8qd2q58vsrjhkzjgtj8nrpezuz4zfuq49fwz5a7e0"),
    ("m/44'/1022'/10'/525'/1460'/2'", "account_tdx_a_128y22qvaswpshcdxqxnspqa52ywmt0lm3m6d9mlqd6n0emqkdcq2tq"),
    ("m/44'/1022'/11'/525'/1460'/3'", "account_tdx_b_12907aun2hnh6aw9fn3e6g22ua3w75amq8yvtdxkmct9sx68d23f8d7"),
    ("m/44'/1022'/12'/525'/1460'/3'", "account_tdx_c_12yur4s9ymydfg6xd43t4ygmygl77k57w5n8mkq523shc00fw8wqf6z"),
    ("m/44'/1022'/13'/525'/1678'/0'", "account_tdx_d_12yvlfgwsnnajytehql0et9evdtytgpt2xp2whkdn7tscucns0slaq6"),
    ("m/44'/1022'/14'/525'/1678'/1'", "account_tdx_e_128czm9huas70jukyvj8dkvpa0n7vthhn8ymchrc97lgg2aee8mmxn4"),
    ("m/44'/1022'/32'/525'/1678'/2'", "account_tdx_20_1280naf7htu2fapvpplsdw9najqsm0kfpnsgld2zpwcyzwhc5222yv5"),
    ("m/44'/1022'/33'/525'/1678'/2'", "account_tdx_21_12y6nghc9383er7e70fvjawjn20yfvfexa2a4hcqjz5v82zx3uchh98"),
    ("m/44'/1022'/34'/525'/1678'/2'", "account_tdx_22_12y67rnvgsyc9rgn8mh5lyjquuxucpvxdpcj005lvn950a8enlx9lwp"),
    ("m/44'/1022'/35'/525'/1678'/2'", "account_tdx_23_129uuanuqz6xkd2mrcc0trrt2pddw78kf886xz42ajyt2e3hkvvq396"),
    ("m/44'/1022'/36'/525'/1678'/2'", "account_tdx_24_129ghwy8xz5eskevskaltrtqrh80em9ujwvc9tklxlw2pzjca58qfxp"),
    ("m/44'/1022'/37'/525'/1678'/2'", "account_tdx_25_1289hhqh8xmv50lvmtuz20y7z24a5ujm0n2ehjq04gj32hr5cj2669c"),
    ("m/44'/1022'/240'/525'/1678'/2'", "account_loc1286jt5l6fw3dxwth4uvv299vx8sz072mjg93e47lgqmugsdphu5eem"),
    ("m/44'/1022'/241'/525'/1678'/2'", "account_test12952h3g26hjc6qwqpspg4fmmmmunvast7n99nmesfeehxrwtymndkk"),
    ("m/44'/1022'/242'/525'/1678'/2'", "account_sim1288duwxpwa7fpxldejl7v8yfqucq8vl04dpn9mpdl44cp3gtxefkel"),
]


# def test_verify_address_ed25519(backend):
#     for vector in test_vectors:
#         call_and_check(backend, vector[0], vector[1])

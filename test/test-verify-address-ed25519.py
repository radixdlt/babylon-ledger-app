# Test GetPubKeyEd25519 instruction
# WARNING: Requires device configured for development (see root README.md)

import sys
import os

from ledgerblue.comm import getDongle
from ledgerblue.commTCP import getDongle as getDongleTCP


# --------------------------------------------------------------------------------------------
# Encode absolute BIP32 path into hex string representation
# --------------------------------------------------------------------------------------------
def encodeBip32(path):
    elements = path.replace('H', "'").replace('"', "'").split('/')
    result = (len(elements) - 1).to_bytes(1, 'little').hex()
    for i in range(1, len(elements)):
        num = 0x80000000 if elements[i].endswith("'") else 0
        num += int(elements[i].replace("'", ""))
        result += num.to_bytes(4, 'big').hex()
    return result


# --------------------------------------------------------------------------------------------
#
# --------------------------------------------------------------------------------------------

def call_and_check(path, expected_pub_key):
    data = encodeBip32(path)
    data_length = int(len(data) / 2).to_bytes(1, 'little').hex()
    response = dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + data_length + data))
    pk = response.decode('utf-8')[1:]
    assert pk == expected_pub_key, "Invalid address\nExpected: " + expected_pub_key + "\nReceived: " + pk
    print(path, " - Success")


# --------------------------------------------------------------------------------------------
# disable printing stack trace
sys.tracebacklimit = 0

if os.environ.get('USE_SPECULOS') is not None:
    dongle = getDongleTCP(debug=False)
else:
    dongle = getDongle(False)

instructionClass = "AA"
instructionCode = "81"
p1 = "00"
p2 = "00"

test_vectors = [
    ("m/44H/1022H/1H/525H/1460H/0H", "account_rdx12939q7jvjc0q9vvqtc87uv6eu334yagtak7k9udekafy66gpvu222n"),
    ("m/44H/1022H/2H/525H/1460H/1H", "account_tdx_2_12y40zqyxrt27h8qd2q58vsrjhkzjgtj8nrpezuz4zfuq49fwz5a7e0"),
    ("m/44H/1022H/10H/525H/1460H/2H", "account_tdx_a_128y22qvaswpshcdxqxnspqa52ywmt0lm3m6d9mlqd6n0emqkdcq2tq"),
    ("m/44H/1022H/11H/525H/1460H/3H", "account_tdx_b_12907aun2hnh6aw9fn3e6g22ua3w75amq8yvtdxkmct9sx68d23f8d7"),
    ("m/44H/1022H/12H/525H/1460H/3H", "account_tdx_c_12yur4s9ymydfg6xd43t4ygmygl77k57w5n8mkq523shc00fw8wqf6z"),
    ("m/44H/1022H/13H/525H/1678H/0H", "account_tdx_d_12yvlfgwsnnajytehql0et9evdtytgpt2xp2whkdn7tscucns0slaq6"),
    ("m/44H/1022H/14H/525H/1678H/1H", "account_tdx_e_128czm9huas70jukyvj8dkvpa0n7vthhn8ymchrc97lgg2aee8mmxn4"),
    ("m/44H/1022H/32H/525H/1678H/2H", "account_tdx_20_1280naf7htu2fapvpplsdw9najqsm0kfpnsgld2zpwcyzwhc5222yv5"),
    ("m/44H/1022H/33H/525H/1678H/2H", "account_tdx_21_12y6nghc9383er7e70fvjawjn20yfvfexa2a4hcqjz5v82zx3uchh98"),
    ("m/44H/1022H/34H/525H/1678H/2H", "account_tdx_22_12y67rnvgsyc9rgn8mh5lyjquuxucpvxdpcj005lvn950a8enlx9lwp"),
    ("m/44H/1022H/35H/525H/1678H/2H", "account_tdx_23_129uuanuqz6xkd2mrcc0trrt2pddw78kf886xz42ajyt2e3hkvvq396"),
    ("m/44H/1022H/36H/525H/1678H/2H", "account_tdx_24_129ghwy8xz5eskevskaltrtqrh80em9ujwvc9tklxlw2pzjca58qfxp"),
    ("m/44H/1022H/37H/525H/1678H/2H", "account_tdx_25_1289hhqh8xmv50lvmtuz20y7z24a5ujm0n2ehjq04gj32hr5cj2669c"),
    ("m/44H/1022H/240H/525H/1678H/2H", "account_loc1286jt5l6fw3dxwth4uvv299vx8sz072mjg93e47lgqmugsdphu5eem"),
    ("m/44H/1022H/241H/525H/1678H/2H", "account_test12952h3g26hjc6qwqpspg4fmmmmunvast7n99nmesfeehxrwtymndkk"),
    ("m/44H/1022H/242H/525H/1678H/2H", "account_sim1288duwxpwa7fpxldejl7v8yfqucq8vl04dpn9mpdl44cp3gtxefkel"),
]

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

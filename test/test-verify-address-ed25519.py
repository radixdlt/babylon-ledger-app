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
    print("Success")


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
    ("m/44H/1022H/12H/525H/1460H/0H", "account_tdx_c_129wjagjzxltd0clr3q4z7hqpw5cc7weh9trs4e9k3zfwqpj6aw829s"),
    ("m/44H/1022H/12H/525H/1460H/1H", "account_tdx_c_1287srsd8ldcueyjjcxflqzpxrgs8dwlhhl9jyyzy6cw26r7qycs5qj"),
    ("m/44H/1022H/12H/525H/1460H/2H", "account_tdx_c_12yd08xvsc6rqwdwmrjll5a0ugzfzr99zwm58zh65jsl8f280wgtrk2"),
    ("m/44H/1022H/12H/525H/1460H/3H", "account_tdx_c_12yur4s9ymydfg6xd43t4ygmygl77k57w5n8mkq523shc00fw8wqf6z"),
    ("m/44H/1022H/12H/525H/1678H/0H", "account_tdx_c_12xf0g2xs2a8up5mch0a273yjd94vxtkyx44uzjdkqm7mypmrw9magu"),
    ("m/44H/1022H/12H/525H/1678H/1H", "account_tdx_c_12902ug8fr0mdh4gttxjkhsfkdfh53v9lqj24z49ymgs8twvqpxaur4"),
    ("m/44H/1022H/12H/525H/1678H/2H", "account_tdx_c_12y9w6f8a8zuy4vt7kldu7udt966dfvuglpl22yutm57vu4yxyt5xz7"),
    ("m/44H/1022H/12H/525H/1678H/3H", "account_tdx_c_128465jurewnupzlug9gwad5czt3h7tptfv0vr9r3v9a6c6acs4r7d6"),
    ("m/44H/1022H/12H/525H/1391H/0H", "account_tdx_c_129cdsjl8a3n7xnr05er0f4zaleu0uguu73p2v8fveu2nsaptcestrk"),
    ("m/44H/1022H/12H/525H/1391H/1H", "account_tdx_c_12yqa0l3g5y3lred64syu3vp7ws67zackdsl2d5pu92yzcqn9dcg9ww"),
    ("m/44H/1022H/12H/525H/1391H/2H", "account_tdx_c_129x0l7k27j7khhj8s4vqha4he769608mve8yts0wmexzz3u57tdn3u"),
    ("m/44H/1022H/12H/525H/1391H/3H", "account_tdx_c_12xm9v8pq07xwrq2rl002dfs87ueufsplysd05m20xsz5qs8xl3rj2w"),
]

print("Testing", "VerifyAddressEd25519", instructionCode)

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

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
    pk = response.hex()
    assert pk == expected_pub_key, "Invalid public key\nExpected: " + expected_pub_key + "\nReceived: " + pk


# --------------------------------------------------------------------------------------------
# disable printing stack trace
sys.tracebacklimit = 0

if os.environ.get('USE_SPECULOS') is not None:
    dongle = getDongleTCP(debug=False)
else:
    dongle = getDongle(False)

instructionClass = "AA"
instructionCode = "21"
p1 = "00"
p2 = "00"

test_vectors = [
    ("m/44H/1022H/10H/525H/1238H/0H", "cffce054df51fb4072e7faf627e0f64f168fd8811f749d34720ac8da264bac06"),
    ("m/44H/1022H/10H/525H/1238H/1H", "e6183ea89d5fc68f5a6d55c3a764476a488f3ed4280c097332bd6e25507190d8"),
    ("m/44H/1022H/10H/525H/1238H/2H", "c5dfa80c836068376fa08a0d45131a786209253ece2c2a9dcccc2a443c55484c"),
    ("m/44H/1022H/10H/618H/1238H/0H", "32b3ea30b734dc92f6e0ce8841b7d3a796d65c82fea1ad8499823ef8214d4942"),
    ("m/44H/1022H/10H/618H/1238H/1H", "d77b3efd33b1b1d3ad53e9ae82532113231c6a48ece77e4cf00f36f28a34e13d")
]

print("Testing", "GetPubKeyEd25519", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

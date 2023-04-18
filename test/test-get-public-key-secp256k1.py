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
instructionCode = "31"
p1 = "00"
p2 = "00"

test_vectors = [
    ("m/44H/1022H/10H/525H/0H", "02e7086602c12f1aca68937770e9790c94d32e7480a5c91902be9464407319bdd6"),
    ("m/44H/1022H/10H/525H/1H", "03cbeb7fc171e7e2a52ec48ef626e28fa4b54b5169ff3037cbbd96b6ef87783cd5"),
    ("m/44H/1022H/10H/525H/2H", "026b81695755c5e69686659f4acf3a23e1e7ae545b772c12c820124a713fff56af"),
    ("m/44H/1022H/10H/618H/0H", "0249dae132729eb825bb186e90a5e93b58f49cc434b6991ccc191368699455a89a"),
    ("m/44H/1022H/10H/618H/1H", "02e0018a83bd3f64457a8f6cf26e34d9e0e1a33f05480633fee6b9b88a9ec25cf6")
]

print("Testing", "GetPubKeySecp256k1", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

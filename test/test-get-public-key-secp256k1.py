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
    ("m/44H/1022H/10H/525H/0H/1238H", "03e6e5f34b265cca342ac711e68b5df9d839bc722e0b004f471539867d179d57c8"),
    ("m/44H/1022H/10H/525H/1H/1238H", "038d9d63a9725d285ccf858b6138edf861e5fac8faf146dbae7dd3c745d38c2116"),
    ("m/44H/1022H/10H/525H/2H/1238H", "03730e6ad9d4952265b35caeda3225a9aa97305d46e4dc20a4b3a81537c068b6b6"),
    ("m/44H/1022H/10H/618H/0H/1238H", "025bc555493dc408d5504d955868a18fbeae855efc335337901376af4d6e9f87f2"),
    ("m/44H/1022H/10H/618H/1H/1238H", "0223252b481dbcae2c8ccec6eb8e6355c8293a140786e519d6bd0caed68d7dee8a")
]

print("Testing", "GetPubKeySecp256k1", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

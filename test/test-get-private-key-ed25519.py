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
instructionCode = "22"
p1 = "00"
p2 = "00"

path = "m/44H/1022H/10H/525H/0H/1238H"
expected_pk = ""

test_vectors = [
    ("m/44H/1022H/10H/525H/0H/1238H", "1545d0732f68aa3060de84e58f8835d95e60959ffcb3c4f3ade141ea34344030"),
    ("m/44H/1022H/10H/525H/1H/1238H", "e5d1e1379c72519e9313b6243ce5713318b2fd7692f21212d219ccc00fc7c5e5"),
    ("m/44H/1022H/10H/525H/2H/1238H", "8d19e82fe69a0954bd84b1b57585232bb2f81c76d1a573db7c755236fabb6853"),
    ("m/44H/1022H/10H/618H/0H/1238H", "8706d1eb164172573a8b8795f760f29df44d82ecc7751fdc48813118558724b2"),
    ("m/44H/1022H/10H/618H/1H/1238H", "389055dee5b54cffe308f3a6a3dcb9cc8cfa36112c6f0b8d570c5c31aa82adb4")
]

print("Testing", "GetPrivKeyEd25519", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

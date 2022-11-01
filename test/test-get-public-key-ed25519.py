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

path = "m/44H/1022H/10H/525H/0H/1238H"
expected_pk = ""

test_vectors = [
    ("m/44H/1022H/10H/525H/0H/1238H", "191a2f9a10b2370ae612efbce92725164a9e3c16907c8eb93b6ded49f69aaf14"),
    ("m/44H/1022H/10H/525H/1H/1238H", "dbee518731b590c92f8fa1c405d3642eb54e7b62364f879348d46e9a31ad372f"),
    ("m/44H/1022H/10H/525H/2H/1238H", "a702d2578e7a1da476bd518445603c706158a34ff1f5ac3763ee069525bc99d5"),
    ("m/44H/1022H/10H/618H/0H/1238H", "298d9d389e8f8e7fea12578cfc9a0a6f21d132a138966d65d12dd5897b90332e"),
    ("m/44H/1022H/10H/618H/1H/1238H", "28e5bcf719b62b40d5a601facb309507057f03bf9ad275a13615570af1e1ecc9")
]

print("Testing", "GetPubKeyEd25519", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

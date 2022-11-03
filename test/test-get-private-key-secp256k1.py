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
instructionCode = "32"
p1 = "00"
p2 = "00"

test_vectors = [
    ("m/44H/1022H/10H/525H/0H/1238H", "299927f75ffcaa22e74c0fdb51bb445899778db14b99f8bd7cae993fd8af467d"),
    ("m/44H/1022H/10H/525H/1H/1238H", "d182ad8a1b1c51bc1d090730909aea8b5a7f0a752eb2b27dcf6a5ab1c5f8de89"),
    ("m/44H/1022H/10H/525H/2H/1238H", "4582f8f14a96c1431779b7ab12550544884f0fb2fabaea36b82215cab212bd3e"),
    ("m/44H/1022H/10H/618H/0H/1238H", "4d18d4a6b845452f3b9d9ec6abbcaf5ff1df256537ea6e0b3f3db8ded7ff9270"),
    ("m/44H/1022H/10H/618H/1H/1238H", "7cb24e0b4d3db03e34b92290dae82e51b4fbc4762e89a5d8eeb630ad6659dfa4")
]

print("Testing", "GetPrivKeySecp256k1", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

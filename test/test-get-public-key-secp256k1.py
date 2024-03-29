# Test GetPubKeySecp256k1 instruction
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
    ("m/44H/1022H/0H/0/0H", "03f43fba6541031ef2195f5ba96677354d28147e45b40cde4662bec9162c361f55"),
    ("m/44H/1022H/0H/0/1H", "0206ea8842365421f48ab84e6b1b197010e5a43a527952b11bc6efe772965e97cc"),
    ("m/44H/1022H/0H/0/2H", "024f44df0493977fcc5704c00c5c89932d77a9a0b016680e6a931684e160fb8d99"),
    ("m/44H/1022H/0H/0/3H", "0388485f6889d7ebcf1cf6f6dafc8ae5d224f9e847fac868c2e006e71ff3383a91"),
    ("m/44H/1022H/0H/0/4H", "024128185f801aee4ebe9a70d6290f60051162526551240da1374363b58e2e1e2c"),
    ("m/44H/1022H/0H/0/5H", "03f3f51a028cbed1a2c0c3b1f21abc5354f58e8d5279e817195750b8ddec9334f4"),
    ("m/44H/1022H/0H/0/6H", "0383d0721aac0569c37edafe5edd6e2d320822dc23f9207b263b2319837ed1a89d"),
    ("m/44H/1022H/0H/0/7H", "03af13461247c39e54fab62597701ab06c67edac7f8de4df1283a2645706c0b153"),
    ("m/44H/1022H/0H/0/8H", "0226912f5226f4a7c9c80780f32c6ad406c8b471c4929033e5e1756ca248c5a278"),
    ("m/44H/1022H/0H/0/9H", "035a9825274e30ce325cc3934b4e23b008097bd97f1b0a0ef57f7dc9a33e5760ed"),
    ("m/44H/1022H/0H/0/0", "03bc2ec8f3668c869577bf66b7b48f8dee57b833916aa70966fa4a5029b63bb18f"),
    ("m/44H/1022H/0H/0/1", "03c8a6a5710b5abba09341c24382de3222913120dee5084e887529bf821f3973e2"),
    ("m/44H/1022H/0H/0/2", "02d6b5b132e16160d6337d83408165c49edac7bb0112b1d1b3e96e3f6908f6d0d6"),
    ("m/44H/1022H/0H/0/3", "03ce5f85ad86922fbc217806a79d9f4d8d6a268f3822ffed9533a9fff73a4374b7"),
    ("m/44H/1022H/0H/0/4", "03e2c66201fc7330992d316d847bdbeb561704b70779ce60a4fcff53ffe5b6cb36"),
    ("m/44H/1022H/0H/0/5", "02df71a292057d1f7cda4fbcd252e43907646610cc191b6f44050683f82a7e63de"),
    ("m/44H/1022H/0H/0/6", "03d054f1c3d7982994d9581c496f84b6cdf584c8eff0401da82d8c19ad88e8a768"),
    ("m/44H/1022H/0H/0/7", "03ccf3b2bd4294d7e7e84268003f1e25c4893a482e28fcf00dfc1ff65679541d50"),
    ("m/44H/1022H/0H/0/8", "0284c067d070bfdb790883cab583f13a0b13f9d52eacdb52a5d8588231ce8c8b89"),
    ("m/44H/1022H/0H/0/9", "02e5703b668deebac710118df687296e90da93c19d0db3cce656b6a677ab3e4747"),
]

print("Testing", "GetPubKeySecp256k1", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

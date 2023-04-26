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
    ("m/44H/1022H/0H/0/0H", "e6aec3c1b9c6b49f154c99708ce4bdb36a01de3f13a832111d7d64e368f939ce"),
    ("m/44H/1022H/0H/0/1H", "7938cc222877aa0f9b4293478bf5733577ceb54cc4834e6e518cbc3847751fd6"),
    ("m/44H/1022H/0H/0/2H", "4e71fa6a8c4612e79b2a9f0f4956f0a8eacf3f19ec56b6d40cc874ac4010f917"),
    ("m/44H/1022H/0H/0/3H", "2ab7a40d7da148a98d200cf2f3a9b6f95c29e637b4d959928f6f315ad2bc0384"),
    ("m/44H/1022H/0H/0/4H", "4c25b3cacca546e8682438e2915698d0ab388abd4e4247d7096814c6ac8f3015"),
    ("m/44H/1022H/0H/0/5H", "318ffcfdf4c40964243f018363017c4749f2503fe948d189d83069684207e8c2"),
    ("m/44H/1022H/0H/0/6H", "34418351c9f02e0b9b9392e1523c14c16cf4edb9df71dfb13b7a6d7ff73be18e"),
    ("m/44H/1022H/0H/0/7H", "f0d67eaa2ff06afd435346ad5e1fd9ad8b892ac643de7bbe07570a152b6b0ffd"),
    ("m/44H/1022H/0H/0/8H", "c5e76f69e4ecef5f590387f1f6d179f65736f77098eebe97e7c8d05c43d4cb0b"),
    ("m/44H/1022H/0H/0/9H", "a019c18261515c004c170ea2d7da1bad2268a1751cd5233842c6ed4c91fa2e77"),
    ("m/44H/1022H/0H/0/0", "623048f7bb88a4d162442b88cdd80c85e4d5933ad9e78523a97de769badb9ab2"),
    ("m/44H/1022H/0H/0/1", "e94b6a64f99a1a143ed570bea9cf896ce82d14f861d0103066e835822037fe6b"),
    ("m/44H/1022H/0H/0/2", "692824fba987bd09ddd42d8ceea38676ed48309d19dc159c4dd4ec83d2a666a1"),
    ("m/44H/1022H/0H/0/3", "9343a365148bdbd0fd8adff8dc1a5f2630b61705141553c5d6d0526da8776f88"),
    ("m/44H/1022H/0H/0/4", "5f288fad35651d1cd3c344d06512593e68ce0c4e6e7f96f1b1fbe337d20b7325"),
    ("m/44H/1022H/0H/0/5", "1289c95c78bcbf21a5455654836b946561d3c477916a049601d4e19cfe7507c2"),
    ("m/44H/1022H/0H/0/6", "c038f504e4fe4999dc9355e1c7547ececa2a8990ab63806e9593cbccfe28fa97"),
    ("m/44H/1022H/0H/0/7", "24333e864e7143ec355bdc9db1405afc1188b2f8bb812075fec34d5459ef7df8"),
    ("m/44H/1022H/0H/0/8", "53e8f579fc1a845357f1ad6194cce8c8e71ede05b3574177d3a31b92950dde5e"),
    ("m/44H/1022H/0H/0/9", "d27012ba6fe8db796c6753790c381b77a86ce058bdbb0e59880d40efad5bdbc3"),
]

print("Testing", "GetPrivKeySecp256k1", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

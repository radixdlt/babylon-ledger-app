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
    ("m/44H/1022H/12H/525H/1460H/0H", "451152a1cef7be603205086d4ebac0a0b78fda2ff4684b9dea5ca9ef003d4e7d"),
    ("m/44H/1022H/12H/525H/1460H/1H", "0a4b894208a1f6b1bd7e823b59909f01aae0172b534baa2905b25f1bcbbb4f0a"),
    ("m/44H/1022H/12H/525H/1460H/2H", "235c27aafa83376d475040a7eb53ea889ae93bda005ef7f445f221f73b43313e"),
    ("m/44H/1022H/12H/525H/1460H/3H", "cc294e8c43be93012f827cd54a11a2f836e5934c2361d61b5a737adbd42bf030"),
    ("m/44H/1022H/12H/525H/1678H/0H", "2612a6865d354ed285baf4877d671276e6cd8cd81e3f1101c35d16853c204fa4"),
    ("m/44H/1022H/12H/525H/1678H/1H", "2f92b0b43ee39c6c3006b2a5c7cdbdee0c6b6835d76a0dc8da0aeffc741d5c96"),
    ("m/44H/1022H/12H/525H/1678H/2H", "3f23bcce53cf2ea14d238f8473aaf3c7ed3f4047fa20158389eabb651766f8d5"),
    ("m/44H/1022H/12H/525H/1678H/3H", "5b36d055cdd07129ba0b780cd285661d5dae02831a55b408f84f9b72ba95c0a9"),
    ("m/44H/1022H/12H/525H/1391H/0H", "d998153796a745c2f733079c791f4ae93eb96a812b39c9ee7a26eca32fa14905"),
    ("m/44H/1022H/12H/525H/1391H/1H", "94e163e6739fa0c9db3f44c0675f185fdb0f1dddb6d929cc49a199717c0a2da2"),
    ("m/44H/1022H/12H/525H/1391H/2H", "9bd51ee27f37367ee4c7cf18e5b8e1b40ae808d3da0350de152c9db34de778d3"),
    ("m/44H/1022H/12H/525H/1391H/3H", "d9d6fc68321ce02c30122c6ff5c1a8068142703f9dac362cff29bfb814a2130f"),
    ("m/44H/1022H/12H/618H/1460H/0H", "cc129e0d00d365f2269cee259923e904b8c46ef5b28aefc18df8ed20ef42a3eb"),
    ("m/44H/1022H/12H/618H/1460H/1H", "f67ac35c37921579b59f77fe05a1012ad8240092c8fed0d8fe1f96206eb99c3e"),
    ("m/44H/1022H/12H/618H/1460H/2H", "02c8074258844ae4b81d80261fc411e95070526e0d92803fe4f343e00dc89ed5"),
    ("m/44H/1022H/12H/618H/1460H/3H", "fca4f791866a48cb53a269e420fa6b7f192e98fee5f9e8f9009962ca3d9baeb2"),
    ("m/44H/1022H/12H/618H/1678H/0H", "f6f2056e3edb8905be1717c1f8f5204242047875ba548c19d42962366800c1d4"),
    ("m/44H/1022H/12H/618H/1678H/1H", "960ee0acd88b0e7f1e8cb171139c2e0e7b8d776134a103b36034a6991fcac175"),
    ("m/44H/1022H/12H/618H/1678H/2H", "07ba2aa69eee065495d8820ef9d5a94b982370e233f04472900cfb5efdb4fa3d"),
    ("m/44H/1022H/12H/618H/1678H/3H", "b4763c9a25d95f32e5ddefc7006ffc4a6570818bf24aeff581ac60cd82a751ba"),
    ("m/44H/1022H/12H/618H/1391H/0H", "996626245f999a4c500c394036db43f73abb18f46970066ff124c750dc096360"),
    ("m/44H/1022H/12H/618H/1391H/1H", "afe925e5aabfa04fb10640cad2c1f6739b3dc9fb4ddeba6ff28e90854d45962d"),
    ("m/44H/1022H/12H/618H/1391H/2H", "1226b881b66c58015e760852c0cb202869b73e29fbe0b3d22d50af0fa606b14a"),
    ("m/44H/1022H/12H/618H/1391H/3H", "7fa1d448ef608a6e1a8533d816367b4fa0d60c39844bb82dbce1ea266105a275"),
]

print("Testing", "GetPubKeyEd25519", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

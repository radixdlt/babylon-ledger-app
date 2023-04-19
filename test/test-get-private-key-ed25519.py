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

test_vectors = [
    ("m/44H/1022H/12H/525H/1460H/0H", "13e971fb16cb2c816d6b9f12176e9b8ab9af1831d006114d344d119ab2715506"),
    ("m/44H/1022H/12H/525H/1460H/1H", "ec7634aff9d698d9a5b4001d5eaa878eefc4fc05939dfedcef0112329fd9966a"),
    ("m/44H/1022H/12H/525H/1460H/2H", "9e96517567abba3e5492db11bc016450abb4e60406038d6423a87a0ad860a9ce"),
    ("m/44H/1022H/12H/525H/1460H/3H", "e92d352f6846e75fb760c40229de8c3c2b04210b2955129877286cf15893a21e"),
    ("m/44H/1022H/12H/525H/1678H/0H", "4a39274fd5f320172329ec96e88b658ad9798ea47f292e30f80915f01f3acc48"),
    ("m/44H/1022H/12H/525H/1678H/1H", "d02147e27720dba12acbddc3d6d4fc43a64cab1dbbdcc3e7e0268c766deeccce"),
    ("m/44H/1022H/12H/525H/1678H/2H", "41519644a280fc18191765ee32fdcc7a37ac95012e18c5fb31679222925da1ce"),
    ("m/44H/1022H/12H/525H/1678H/3H", "b843c520aca4d980f69dcf02a6d3deb50c10bdaa350ed262b49cbb0997fcbd28"),
    ("m/44H/1022H/12H/525H/1391H/0H", "62e1255e91b6fafdf06d3d6e3cfa660a5dd39aaa6ab7c207e2535c86f597e47f"),
    ("m/44H/1022H/12H/525H/1391H/1H", "30d9891cf7436d07f45a3358bbf4b0857388c08d1a7fe9973fd6044fa86a9ce0"),
    ("m/44H/1022H/12H/525H/1391H/2H", "80b136f184159c7873321a73e6523be68d428440f95efb77fb67b43560cd5401"),
    ("m/44H/1022H/12H/525H/1391H/3H", "215af43054ac6055f86c9986d482a8fe6b0bf70543f6ebe74f69e33424b11282"),
    ("m/44H/1022H/12H/618H/1460H/0H", "9c683ba15644596f747bc749fed2657644c2873391f9c874efd32ccacc5adf08"),
    ("m/44H/1022H/12H/618H/1460H/1H", "aa45993887e5fe45252db7b34ad26686a4ef165f65ba30206d87d900310ea360"),
    ("m/44H/1022H/12H/618H/1460H/2H", "a5b3a586440f996d12ac9f21f61ed0758c13c012e42ed8c9d83e4bf4548e3dd3"),
    ("m/44H/1022H/12H/618H/1460H/3H", "da699f61d6c2a4893d00b1f15158894974fb403a16a865583538f0542e883c54"),
    ("m/44H/1022H/12H/618H/1678H/0H", "7997d39b74a390bc213c566ec016dd9023c4319af9da5194fb87c0d73f1d970f"),
    ("m/44H/1022H/12H/618H/1678H/1H", "fa9c15acc1f46b790acdb060682c8d9fca307f02ba7a1deee4009c4f89cc3ddc"),
    ("m/44H/1022H/12H/618H/1678H/2H", "4aea7c10102b93b173a72c62c6e5b3a19dacbc4e5dee6fc3f32e04a35d012059"),
    ("m/44H/1022H/12H/618H/1678H/3H", "11e570fe1fc5c7a0deba1c672428b0793f45ca091580a50561ab46e50147ed07"),
    ("m/44H/1022H/12H/618H/1391H/0H", "603ca94347db5edba67c73fa2c75d40f8534efbb6f043e279a62959b799fc55b"),
    ("m/44H/1022H/12H/618H/1391H/1H", "8564e2302ef419354a47265b6e5e6bed276b34cb691ef5a73fd6a722052cacda"),
    ("m/44H/1022H/12H/618H/1391H/2H", "7af1d0b3f6a634891fb502de1bc14d3a06402e96380dfe377d4fb5864922cdf6"),
    ("m/44H/1022H/12H/618H/1391H/3H", "f5ec1d8379d2173975ea693afbd8940820f9d1b82b9f777f02c1ecd4197deab0"),
]

print("Testing", "GetPrivKeyEd25519", instructionCode, end=" ")

for vector in test_vectors:
    call_and_check(vector[0], vector[1])

print("Success")

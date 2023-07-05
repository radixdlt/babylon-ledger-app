# Test SignAuthSecp256k1 instruction

import sys
import os

from ledgerblue.comm import getDongle
from ledgerblue.commTCP import getDongle as getDongleTCP
from cryptography.hazmat.primitives.asymmetric import ec, utils
from cryptography.hazmat.primitives import hashes

# disable printing stack trace
sys.tracebacklimit = 0

if os.environ.get('USE_SPECULOS') is not None:
    dongle = getDongleTCP(debug=False)
else:
    dongle = getDongle(False)

instructionClass = "AA"
instructionCode = "71"  # SignAuthSecp256k1
p1 = "00"
p2 = "00"
dataLength = "00"

print("Testing", "SignAuth", instructionCode)


def encode_bip32(path):
    elements = path.replace('H', "'").replace('"', "'").split('/')
    result = (len(elements) - 1).to_bytes(1, 'little').hex()
    for i in range(1, len(elements)):
        num = 0x80000000 if elements[i].endswith("'") else 0
        num += int(elements[i].replace("'", ""))
        result += num.to_bytes(4, 'big').hex()
    return result


def send_auth_request(daddr, origin, nonce):
    addr_length = len(daddr).to_bytes(1, 'little').hex()
    data = nonce + addr_length + daddr.encode('utf-8').hex() + origin.encode('utf-8').hex()
    data_length = int(len(data) / 2).to_bytes(1, 'little').hex()

    try:
        rc = dongle.exchange(bytes.fromhex("AC" + instructionCode + p1 + p2 + data_length + data))
    except Exception as e:
        print("Error sending txn chunk: ", e)
        return None
    return rc


def send_derivation_path(bip_path):
    path_data = encode_bip32(bip_path)
    data_length = int(len(path_data) / 2).to_bytes(1, 'little').hex()
    # print("Sending derivation path: ", bip_path, ", data_len = ", data_length)

    try:
        return dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + data_length + path_data))
    except Exception as e:
        print("Error sending derivation path: ", e)
        return None


test_vectors = [
    (
        "dc47fc69e9e45855addf579f398da0309c878092dd95352b9fe187a7e5a529e2",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "ec5dcb3d1f75627be1021cb8890f0e8ce0c9fe7f2ff55cbdff096b38a32612c9",
    ),
    (
        "866836f5b9c827ca38fd2bfef94f95ba21933f75a0291c85d3ecfc18b8aa5b2d",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "d7fb740b9ff00657d710dcbeddb2d432e697fc0dd39c60feb7858b17ef0eff58",
    ),
    (
        "0f41aa92e8c978d7f920ca56daf123a0a0d975eea06ecfb57bec0a0560fb73e3",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "4aaa2ec25c3fe215412b3f005e4c37d518af3a22b4728587cf6dbcf83341e8b3",
    ),
    (
        "9c8d2622cedb9dc4e53daea398dd178a2ec938d402eeaba41a2ac946b0f4dd57",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "a10fad201666b4bcf7f707841d58b11740c290e03790b17ed0fec23b3f180e65",
    ),
    (
        "2c07a4fc72341ae9160a8f9ddf2d0bb8fd9d795ed0d87059a9e5de8321513871",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "718b0eb060a719492011910258a4b4119d8c95aef34eb9519c9fa7de25f7ac43",
    ),
    (
        "306b2407e8b675bb22b630efa938249595433975276862e9bfa07f7f94ca84a8",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "9a4f834aefdc455cb4601337227e1b7e74d60308327564ececf33456509964cd",
    ),
    (
        "a14942b1dc361c7e153e4d4200f902da1dafa2bd54bc4c0387c779c22a1e454e",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "00dca15875839ab1f549445a36c7b5c0dcf7aebfa7d48f945f2aa5cf4aa1a9a3",
    ),
    (
        "6a13329619caafdf4351d1c8b85b7f523ce2955873f003402be6e1e45cdce4ae",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "0a510b2362c9ce19d11c538b2f6a15f62caab6528071eaad5ba8a563a02e01cb",
    ),
    (
        "f9ec8f328d9aeec55546d1cd78a13cc7967bd52aba3c8e305ed39f82465f395c",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "20619c1df905a28e7a76d431f2b59e99dd1a8f386842e1701862e765806a5c47",
    ),
]

for vector in test_vectors:
    send_derivation_path("m/44H/1022H/10H/525H/1238H")
    rc = send_auth_request(vector[1], vector[2], vector[3])

    if rc is None:
        print("Failed")
    else:
        r = int.from_bytes(rc[1:33], byteorder='big', signed=False)
        s = int.from_bytes(rc[33:65], byteorder='big', signed=False)
        signature = utils.encode_dss_signature(int(r), int(s))
        pubkey = ec.EllipticCurvePublicKey.from_encoded_point(ec.SECP256K1(), bytes(rc[65:98]))
        try:
            # Note that Prehashed parameter is irrelevant here, we just need to pass something known to the library
            pubkey.verify(signature, bytes(rc[98:130]), ec.ECDSA(utils.Prehashed(hashes.SHA256())))
            print("Success")
            assert rc[98:130].hex() == vector[0], "Invalid calculated hash\nExpected: " + vector[0] + "\nReceived: " + rc[98:130].hex()
        except Exception as e:
            print("Invalid signature ", e)

print("All tests successfully passed")

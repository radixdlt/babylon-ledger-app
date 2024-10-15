# Test SignTxEd25519 instruction (mode: do not show digest)

import sys
import os
import hashlib

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
instructionCode = "A2"
p1 = "00"
p2 = "00"
dataLength = "00"

print("Testing", "SignPreAuthHashSecp256k1", instructionCode)

test_hash_inputs = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']

def encode_bip32(path):
    elements = path.replace('H', "'").replace('"', "'").split('/')
    result = (len(elements) - 1).to_bytes(1, 'little').hex()
    for i in range(1, len(elements)):
        num = 0x80000000 if elements[i].endswith("'") else 0
        num += int(elements[i].replace("'", ""))
        result += num.to_bytes(4, 'big').hex()
    return result


def send_preauth_hash(data):
    cls = "AC"
    data_length = len(data).to_bytes(1, 'little').hex()
    # print("Sending data_len = ", data_length, " len = ", len(data))

    try:
        return dongle.exchange(bytes.fromhex(cls + instructionCode + p1 + p2 + data_length + data.hex()))
    except Exception as exception:
        print("Error sending txn chunk: ", exception)
        return None

def send_derivation_path(bip_path):
    path_data = encode_bip32(bip_path)
    data_length = int(len(path_data) / 2).to_bytes(1, 'little').hex()
    # print("Sending derivation path: ", bip_path, ", data_len = ", data_length)

    try:
        return dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + data_length + path_data))
    except Exception as exception:
        print("Error sending derivation path: ", exception)
        return None


for inp in test_hash_inputs:
    hash_calculator = hashlib.blake2b(digest_size=32)
    hash_calculator.update(inp)
    send_derivation_path("m/44H/1022H/10H/525H/1238H")
    rc = send_preauth_hash(hash_calculator.digest())

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
        except Exception as e:
            print("Invalid signature ", e)

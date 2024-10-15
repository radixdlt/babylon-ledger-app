# Test SignTxEd25519 instruction (mode: do not show digest)

import sys
import os
import hashlib

from ledgerblue.comm import getDongle
from ledgerblue.commTCP import getDongle as getDongleTCP
from cryptography.hazmat.primitives.asymmetric import ed25519

# disable printing stack trace
sys.tracebacklimit = 0

if os.environ.get('USE_SPECULOS') is not None:
    dongle = getDongleTCP(debug=False)
else:
    dongle = getDongle(False)

instructionClass = "AA"
instructionCode = "A1"
p1 = "00"
p2 = "00"
dataLength = "00"

print("Testing", "SignPreAuthHashEd25519", instructionCode)

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
    send_derivation_path("m/44H/1022H/12H/525H/1460H/0H")
    rc = send_preauth_hash(hash_calculator.digest())

    if rc is None:
        print("Failed")
    else:
        pubkey = ed25519.Ed25519PublicKey.from_public_bytes(bytes(rc[64:96]))
        try:
            pubkey.verify(bytes(rc[0:64]), bytes(rc[96:128]))
            print("Success")
        except Exception as e:
            print("Invalid signature ", e)

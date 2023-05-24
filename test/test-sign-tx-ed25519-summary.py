import sys
import os

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
instructionCode = "42"
p1 = "00"
p2 = "00"
dataLength = "00"

print("Testing", "SignTxEd25519Summary", instructionCode)


def list_files():
    dir_path = "data"
    res = []
    for path in os.listdir(dir_path):
        if os.path.isfile(os.path.join(dir_path, path)):
            res.append(os.path.join(dir_path, path))
    return res


def read_file(file):
    print("Reading ", file)
    with open(file, "rb") as f:
        return f.read()


def encode_bip32(path):
    elements = path.replace('H', "'").replace('"', "'").split('/')
    result = (len(elements) - 1).to_bytes(1, 'little').hex()
    for i in range(1, len(elements)):
        num = 0x80000000 if elements[i].endswith("'") else 0
        num += int(elements[i].replace("'", ""))
        result += num.to_bytes(4, 'big').hex()
    return result


def send_tx_intent(txn):
    num_chunks = len(txn) // 255 + 1
    # print("Sending txn (", len(txn), " bytes, ", num_chunks, " chunk(s))")
    for i in range(num_chunks):
        chunk = txn[i * 255:(i + 1) * 255]
        cls = "AC" if i == num_chunks - 1 else "AB"
        data_length = len(chunk).to_bytes(1, 'little').hex()

        # print("Chunk:", i, "data:", chunk.hex(), "len:", data_length, "cls:", cls)

        try:
            rc = dongle.exchange(bytes.fromhex(cls + instructionCode + p1 + p2 + data_length + chunk.hex()))
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


for file_name in list_files():
    if not file_name.endswith("fer.txn"):
        continue
    data = read_file(file_name)
    send_derivation_path("m/44H/1022H/12H/525H/1460H/0H")
    rc = send_tx_intent(data)

    if rc is None:
        print("Failed")
    else:
        pubkey = ed25519.Ed25519PublicKey.from_public_bytes(bytes(rc[64:96]))
        try:
            pubkey.verify(bytes(rc[0:64]), bytes(rc[96:128]))
            print("Success")
        except Exception as e:
            print("Invalid signature ", e)

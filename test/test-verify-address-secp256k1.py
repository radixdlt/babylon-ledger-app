# Test GetPubKeyEd25519 instruction
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
    pk = response.decode('utf-8')[1:]
    assert pk == expected_pub_key, "Invalid address\nExpected: " + expected_pub_key + "\nReceived: " + pk
    print("Success")


# --------------------------------------------------------------------------------------------
# disable printing stack trace
sys.tracebacklimit = 0

if os.environ.get('USE_SPECULOS') is not None:
    dongle = getDongleTCP(debug=False)
else:
    dongle = getDongle(False)

instructionClass = "AA"
instructionCode = "91"
p1 = "00"
p2 = "00"

test_vectors = [
    ("m/44H/1022H/0H/0/0H", "account_rdx16x5wz8wmkumuhn49klq0zwgjn9d8xs7n95maxam04vawld2d27jjz0"),
    ("m/44H/1022H/0H/0/1H", "account_rdx16y6q3q6ey64j5qvkex3q0yshtln6z2lmyk254xrjcq393rc0hc79wl"),
    ("m/44H/1022H/0H/0/2H", "account_rdx16y69pp98nqvh5xt36mcu78m4pvptpu65wa3nsp9qphn3lc4kp8r66k"),
    ("m/44H/1022H/0H/0/3H", "account_rdx16x9rxepradk8g4zamprt7gn5vhs5j3v5e49yqt0k2w64a6phgf58qa"),
    ("m/44H/1022H/0H/0/4H", "account_rdx16ywu4yg6ka4hd0nmju8cjsls75a98d238na6x0phxll855tjgt4gyk"),
    ("m/44H/1022H/0H/0/5H", "account_rdx1685mjfygmz8e9k3x2ee7pt67kqr28lhayug0yuzv4g2v32uakaf9t0"),
    ("m/44H/1022H/0H/0/6H", "account_rdx16yz4qrctz6843j79rvffunvdaz6l0kem266ddg45cnvw9x4g7hhuv5"),
    ("m/44H/1022H/0H/0/7H", "account_rdx16x743lcavljkrfqs4y2slhwtnj3dyqn0glf4gq96m26ff64kl4rnc2"),
    ("m/44H/1022H/0H/0/8H", "account_rdx169lnw3a4dtxy2jggv80dt2vn8tcyypyqjsver6322vx9januj7dwhu"),
    ("m/44H/1022H/0H/0/9H", "account_rdx16xss3896jatrp7zcxgtuvsjy3cgs22s3wnv0qqcdp79jdkv6sufl79"),
    ("m/44H/1022H/0H/0/0", "account_rdx168cucf0r7h07hhzezrz8uem7lh0xxhgvrlv8htvqwjdxfn2k4jrhtk"),
    ("m/44H/1022H/0H/0/1", "account_rdx168vfmhy37elgswtcqwsnjn7hh906s04fwwmdqpmal8l7cc6gvxw4mj"),
    ("m/44H/1022H/0H/0/2", "account_rdx16x3fhgxm2kl99tyzf3zg4qj7z9csfayzgenqj52aykvltsmyvkcknl"),
    ("m/44H/1022H/0H/0/3", "account_rdx16xp6eppfc7fw0vmnj552hwjdxtwz9ymeynwm5fh5jx6kx76zmhdvan"),
    ("m/44H/1022H/0H/0/4", "account_rdx16xjyhlxdjt7smrwawhnvxaw2arndkan6kvywq6us60aqet4p8tpm58"),
    ("m/44H/1022H/0H/0/5", "account_rdx169gwv4uq7ftrenpf6z2lxxwq2mcqsplw3ghcs4n0g2a868l88yq4a4"),
    ("m/44H/1022H/0H/0/6", "account_rdx1686u3ytx09nuw3c9nyauvx7jzdlcy2xqsz740axumt2k6v4jh00yc9"),
    ("m/44H/1022H/0H/0/7", "account_rdx16yvcymlwp2ym4q9dj2ltzzq0df5vltx0xtcyanna78j0mejwf7gnwr"),
    ("m/44H/1022H/0H/0/8", "account_rdx16ys0zmzfjfrsjlsjh8rpl8x0zj8jwt0l86up7rnyfwmy0rkd3xc8aw"),
    ("m/44H/1022H/0H/0/9", "account_rdx168l3drrzhjnlc9a57hmvt6rykhj5l3hrljpj7juu52rgdcyuey7xft"),
]

print("Testing", "VerifyAddressEd25519", instructionCode)

for vector in test_vectors:
    call_and_check(vector[0], vector[1])






















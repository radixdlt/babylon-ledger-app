import sys
import os
# disable printing stack trace
sys.tracebacklimit = 0

from ledgerblue.comm import getDongle
from ledgerblue.commTCP import getDongle as getDongleTCP

if os.environ.get('USE_SPECULOS') is not None:
    dongle = getDongleTCP(debug=False)
else:
    dongle = getDongle(False)

instructionClass = "AA"
instructionCode = "21"
p1 = "00"
p2 = "00"

# 6 elements + m/44H/1022H/10H/525H/0H/1238H
# 44H -> 8000002c
# 1022H -> 800003fe
#

data = "06" + "8000002C800003FE80000000800000008000000080000000"

print("Data len: (orig) ", int(len(data)/2))

dataLength = int(len(data)/2).to_bytes(1, 'little').hex()

print("Data len: ", dataLength)

# sending message to Ledger
publicKey = dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + dataLength + data))

print("Testing", "GetPubKeyEd25519", instructionCode, end=" ")
response = dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + dataLength))

assert response.hex() == '000001', "Invalid public key"
print("Success")

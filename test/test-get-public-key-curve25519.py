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
instructionCode = "10"
p1 = "00"
p2 = "00"
dataLength = "00"

# 6 elements +
data = "06"

dataLength = int(len(data)/2).to_bytes(4, 'little').hex()
# sending message to Ledger
publicKey = dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + dataLength + data))

print("Testing", "GetPublicKeyCurve25519", instructionCode, end=" ")
response = dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + dataLength))

assert response.hex() == '000001', "Invalid public key"
print("Success")

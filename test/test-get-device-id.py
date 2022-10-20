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
instructionCode = "12"
p1 = "00"
p2 = "00"
dataLength = "00"

print("Testing", "GetDeviceId", instructionCode, end=" ")
response = dongle.exchange(bytes.fromhex(instructionClass + instructionCode + p1 + p2 + dataLength))

print("Result:", response)
assert response.hex() == '000001', "Invalid public key"
print("Success")


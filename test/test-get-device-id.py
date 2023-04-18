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

assert response.hex() == 'ed798e66ded43a63ba7a41cf060062ba4a0c55ad69b14f2215c1526383ac4157aa619ee9106b02582047dd7f802b305dbb0b1641dde02aef7fdac937b9f9ee8c', "Invalid device ID\nReceived:" + response.hex()
print("Success")

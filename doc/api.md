# Babylon Ledger App API

All communication is performed using APDU protocol ([see APDU description](apdu.md)) via USB.

## Overview

| API Name                                        | Instruction Code | Description                                                                                                                                                                                                                                                                               |
|-------------------------------------------------|------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [GetAppVersion](#getappversion)                 | 0x10             | Get application version as 3 bytes, where each byte represents version component: __Major__, __Minor__ and __Patch Level__.                                                                                                                                                               |
| [GetDeviceModel](#getdevicemodel)               | 0x11             | Get device model code byte. __0__ corresponds to Nano S, __1__ - Nano S Plus, __2__ - Nano X                                                                                                                                                                                              |
| [GetDeviceId](#getdeviceid)                     | 0x12             | Get device ID byte array (32 bytes)                                                                                                                                                                                                                                                       |
| [GetPubKeyEd25519](#getpubkeyed25519)           | 0x21             | Get Ed25519 public key for provided derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme.                                                                                                                                                            |
| [GetPrivKeyEd25519](#getprivkeyed25519)         | 0x22             | Get Ed25519 private key for provided derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme. Command available only in debug build.                                                                                                                    |
| [GetPubKeySecp256k1](#getpubkeysecp256k1)       | 0x31             | Get Secp256k1 public key for provided derivation path.                                                                                                                                                                                                                                    |
| [GetPrivKeySecp256k1](#getprivkeysecp256k1)     | 0x32             | Get Secp256k1 private key for provided derivation path. Command available only in debug build.                                                                                                                                                                                            |
| [SignTxEd25519](#signtxed25519)                 | 0x41             | Sign transaction intent using Ed25519 curve and given derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme. Signing is done in "advanced mode", when every instruction from transaction intent is decoded and displayed to user.                     |
| [SignTxEd25519Summary](#signtxed25519summary)   | 0x42             | Sign transaction intent using Ed25519 curve and given derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme. Signing is performed in "summary mode", when device tries to recognize known transaction format and provide summary for the transaction. |
| [SignTxSecp256k1](#signtxsecp256k1)             | 0x51             | Sign transaction intent using Secp256k1 curve and given derivation path. Signing is done in "advanced mode", when every instruction from transaction intent is decoded and displayed to user.                                                                                             |
| [SignTxSecp256k1Smart](#signtxsecp256k1summary) | 0x52             | Sign transaction intent using Secp256k1 curve and given derivation path. Signing is performed in "summary mode", when device tries to recognize known transaction format and provide summary for the transaction.                                                                         |
| [SignAuthEd25519](#signauthed25519)             | 0x61             | Sign provided 32 bytes digest using Ed25519 curve and given derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme.                                                                                                                                    |

## GetAppVersion

APDU:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x10 | 0x00 | 0x00 | None |

Response (3 bytes):
| Data | Description |
|------|-------------|
| byte 0| Major application version |
| byte 1| Minor application version |
| byte 2| Patch level |

## GetDeviceModel

APDU:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x11 | 0x00 | 0x00 | None |

Response (1 byte):
| Data | Description |
|------|-------------|
| byte 0| Device model code byte. 0 corresponds to Ledger Nano S, 1 - Ledger Nano S+, 2 - Ledger Nano X |

## GetDeviceId

APDU:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x12 | 0x00 | 0x00 | None |

Response (64 bytes):
| Data | Description |
|------|-------------|
| byte 0-63 | Device ID byte array |

## GetPubKeyEd25519
__Note__: This command accepts derivation path in the format described in CAP-26 SLIP 10 HD Derivation Path Scheme.

APDU:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x21 | 0x00 | 0x00 | Derivation path in the following format:    
byte 0 - number of elements in derivation path    
bytes 1-5 - first element of derivation path in big endian format   
bytes 6-9 - second element of derivation path in big endian format   
... - remaining elements of derivation path|

Response (32 bytes):
| Data | Description |
|------|-------------|
| byte 0-31 | Ed25519 public key |

## GetPrivKeyEd25519
__WARNING!!!__ This command is available only in debug build and intended only for debugging purposes. Production build does not support this command and returns error code 0x6EFF (Not Implemented).
__Note__: This command accepts derivation path in the format described in CAP-26 SLIP 10 HD Derivation Path Scheme.

APDU:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x22 | 0x00 | 0x00 | Derivation path in the following format:    
byte 0 - number of elements in derivation path    
bytes 1-5 - first element of derivation path in big endian format   
bytes 6-9 - second element of derivation path in big endian format   
... - remaining elements of derivation path|

Response (32 bytes):
| Data | Description |
|------|-------------|
| byte 0-31 | Ed25519 private key |

## GetPubKeySecp256k1

APDU:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x31 | 0x00 | 0x00 | Derivation path in the following format:    
byte 0 - number of elements in derivation path    
bytes 1-5 - first element of derivation path in big endian format   
bytes 6-9 - second element of derivation path in big endian format   
... - remaining elements of derivation path|

Response (33 bytes):
| Data | Description |
|------|-------------|
| byte 0-32 | Secp256k1 public key |

Returned key is a compressed key. First byte is always 0x02 or 0x03, depending on the parity of the y-coordinate. 
The remaining 32 bytes are the x-coordinate.

## GetPrivKeySecp256k1
__WARNING!!!__ This command is available only in debug build and intended only for debugging purposes. Production build does not support this command and returns error code 0x6EFF (Not Implemented).

APDU:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x32 | 0x00 | 0x00 | Derivation path in the following format:    
byte 0 - number of elements in derivation path    
bytes 1-5 - first element of derivation path in big endian format   
bytes 6-9 - second element of derivation path in big endian format   
... - remaining elements of derivation path|

Response (32 bytes):
| Data | Description |
|------|-------------|
| byte 0-31 | Secp256k1 private key |

## SignTxEd25519
__Note__: This command accepts derivation path in the format described in CAP-26 SLIP 10 HD Derivation Path Scheme.

This command is invoked in two steps:  
- Send derivation path. The request format is the same as for GetPubKeyEd25519 command except different instruction code (see below).
- Send transaction intent data (see below). This command can be sent one or more times, depending on the size of the transaction intent. The last chunk is accompanied with class byte set to `0xAC`. Other (intermediate) chunks are accompanied with class byte set to `0xAD`.

APDU for derivation path:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x41 | 0x00 | 0x00 | Derivation path in the following format:
byte 0 - number of elements in derivation path  
bytes 1-5 - first element of derivation path in big endian format  
bytes 6-9 - second element of derivation path in big endian format  
... - remaining elements of derivation path|

APDU for transaction intent data:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAD - for intermediate chunks  
  0xAC - for last chunk | 0x41 | 0x00 | 0x00 | Transaction intent data chunk |

Upon successful sign, the device returns the signature for the transaction intent. The signature is returned in the following format:
| Data | Description |
|------|-------------|
| byte 0-63 | Ed25519 signature |
| byte 64-95 | Ed25519 public key |
| byte 96-127 | Calculated digest |

If user rejects the sign request, then the device returns error code 0x6e50 (User rejected the sign request).

## SignTxEd25519Summary
__Note__: This command accepts derivation path in the format described in CAP-26 SLIP 10 HD Derivation Path Scheme.

This command accepts the same data as [SignTxEd25519](#signtxed25519) command and returns the same results. The only difference between commands is the
interaction with the user. The [SignTxEd25519](#signtxed25519) always displays all instructions present in the transaction intent, 
while [SignTxEd25519Summary](#signtxed25519summary) tries to recognize type of the transaction and provide summary for the transaction.
For example, for the payment transaction, the device displays only source and destination accounts and amount of the payment.

## SignTxSecp256k1

This command is invoked in two steps:
- Send derivation path. The request format is the same as for GetPubKeyEd25519 command except different instruction code (see below) and accepted path length.
- Send transaction intent data (see below). This command can be sent one or more times, depending on the size of the transaction intent. The last chunk is accompanied with class byte set to `0xAC`. Other (intermediate) chunks are accompanied with class byte set to `0xAD`.

APDU for derivation path:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x51 | 0x00 | 0x00 | Derivation path in the following format:
byte 0 - number of elements in derivation path  
bytes 1-5 - first element of derivation path in big endian format  
bytes 6-9 - second element of derivation path in big endian format  
... - remaining elements of derivation path|

APDU for transaction intent data:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAD - for intermediate chunks  
0xAC - for last chunk | 0x41 | 0x00 | 0x00 | Transaction intent data chunk |

Upon successful sign, the device returns the signature for the transaction intent. The signature is returned in the following format:
| Data | Description |
|------|-------------|
| byte 0-64 | Secp256k1 signature |
| byte 65-97 | Secp256k1 public key |
| byte 96-127 | Calculated digest |

If user rejects the sign request, then the device returns error code 0x6e50 (User rejected the sign request).

## SignTxSecp256k1Summary
This command accepts the same data as [SignTxSecp256k1](#signtxsecp256k1) command and returns the same results. The only difference between commands is the
interaction with the user. The [SignTxSecp256k1](#signtxsecp256k1) always displays all instructions present in the transaction intent, while 
[SignTxSecp256k1Summary](#signtxsecp256k1summary) tries to recognize type of the transaction and provide summary for the transaction.
For example, for the payment transaction, the device displays only source and destination accounts and amount of the payment.

## SignAuthEd25519
__Note__: This command accepts derivation path in the format described in CAP-26 SLIP 10 HD Derivation Path Scheme.

This command is invoked in two steps:
- Send derivation path. The request format is the same as for GetPubKeyEd25519 command except different instruction code (see below).
- Send auth request data (nonce, origin and dApp address). This packet uses class byte set to `0xAC`.

APDU for derivation path:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAA | 0x61 | 0x00 | 0x00 | Derivation path in the following format:
byte 0 - number of elements in derivation path  
bytes 1-5 - first element of derivation path in big endian format  
bytes 6-9 - second element of derivation path in big endian format  
... - remaining elements of derivation path|

APDU for auth request data:
| CLA | INS | P1 | P2 | Data |
|-----|-----|----|----|------|
| 0xAC | 0x61 | 0x00 | 0x00 | Auth request data in the following format:
byte 0-31 - nonce
byte 32 - dApp address length
byte 33-... - dApp address
byte ... - bytes left after dApp address contains origin|

Upon successful sign, the device returns the signature for the given auth request. The signature is returned in the following format:
| Data | Description |
|------|-------------|
| byte 0-63 | Ed25519 signature   |
| byte 64-95 | Ed25519 public key |
| byte 96-127 | Calculated digest |

If user rejects the sign request, then the device returns error code 0x6e50 (User rejected the sign request).

# Babylon Ledger App API

All communication is performed using APDU protocol ([see APDU description](apdu.md)) via USB.

## Overview

| API Name                                          | Instruction Code | Description                                                                                                                                                                                                                                                           |
|---------------------------------------------------|------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [GetAppVersion](#getappversion)                   | 0x10             | Get application version as 3 bytes, where each byte represents version component: __Major__, __Minor__ and __Patch Level__.                                                                                                                                           |
| [GetDeviceModel](#getdevicemodel)                 | 0x11             | Get device model code byte. __0__ corresponds to Nano S, __1__ - Nano S Plus, __2__ - Nano X                                                                                                                                                                          |
| [GetDeviceId](#getdeviceid)                       | 0x12             | Get device ID byte array (32 bytes)                                                                                                                                                                                                                                   |
| [GetAppSettings](#getappsettings)                 | 0x20             | Get application settings                                                                                                                                                                                                                                              |
| [GetPubKeyEd25519](#getpubkeyed25519)             | 0x21             | Get Ed25519 public key for provided derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme.                                                                                                                                        |
| [GetPubKeySecp256k1](#getpubkeysecp256k1)         | 0x31             | Get Secp256k1 public key for provided derivation path.                                                                                                                                                                                                                |
| [SignTxEd25519](#signtxed25519)                   | 0x41             | Sign transaction intent using Ed25519 curve and given derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme. Signing is done in "advanced mode", when every instruction from transaction intent is decoded and displayed to user. |
| [SignTxSecp256k1](#signtxsecp256k1)               | 0x51             | Sign transaction intent using Secp256k1 curve and given derivation path. Signing is done in "advanced mode", when every instruction from transaction intent is decoded and displayed to user.                                                                         |
| [SignAuthEd25519](#signauthed25519)               | 0x61             | Sign provided 32 bytes digest using Ed25519 curve and given derivation path. Derivation path must conform to CAP-26 SLIP 10 HD Derivation Path Scheme.                                                                                                                |
| [SignAuthSecp256k1](#signauthsecp256k1)           | 0x71             | Sign provided 32 bytes digest using Secp265k1 curve and given derivation path.                                                                                                                                                                                        |
| [VerifyAddressEd25519](#verifyaddressed25519)     | 0x81             | Verify bech32m address for a given derivation path for Ed25519 curve.                                                                                                                                                                                                 |
| [VerifyAddressSecp256k1](#verifyaddresssecp256k1) | 0x91             | Verify bech32m address for a given derivation path for Secp256k1 curve.                                                                                                                                                                                               |

## GetAppVersion

Get application version.

APDU:

| CLA  | INS  | P1   | P2   | Data |
|------|------|------|------|------|
| 0xAA | 0x10 | 0x00 | 0x00 | None |

Response (3 bytes):

| Data   | Description               |
|--------|---------------------------|
| byte 0 | Major application version |
| byte 1 | Minor application version |
| byte 2 | Patch level               |

## GetDeviceModel

Get device model information.

APDU:

| CLA  | INS  | P1   | P2   | Data |
|------|------|------|------|------|
| 0xAA | 0x11 | 0x00 | 0x00 | None |

Response (1 byte):

| Data   | Description                                                            |
|--------|------------------------------------------------------------------------|
| byte 0 | Device model code byte:<br>0 - Nano S<br>1 - Nano S Plus<br>2 - Nano X |

## GetDeviceId

Get ID of the device. Note that this ID is derived from the device's private key. Two devices with same private key will
have same device ID.

APDU:

| CLA  | INS  | P1   | P2   | Data |
|------|------|------|------|------|
| 0xAA | 0x12 | 0x00 | 0x00 | None |

Response (64 bytes):

| Data      | Description          |
|-----------|----------------------|
| byte 0-63 | Device ID byte array |

## GetAppSettings

Get application settings.

APDU:

| CLA  | INS  | P1   | P2   | Data |
|------|------|------|------|------|
| 0xAA | 0x20 | 0x00 | 0x00 | None |

Response (2 bytes):

| Data   | Description                                                                          |
|--------|--------------------------------------------------------------------------------------|
| byte 0 | Verbose mode state<br>0 - Verbose mode disabled<br>1 - Verbose mode enabled          |
| byte 1 | "Blind signing" state<br>0 - "Blind signing" disabled<br>1 - "Blind signing" enabled |

## GetPubKeyEd25519

Get Ed25519 public key for provided derivation path. Derivation path should follow the format described in CAP-26 SLIP
10 HD Derivation Path Scheme.

APDU:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                 |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x21 | 0x00 | 0x00 | Derivation path in the following format:<br>byte 0 - number of elements in derivation path<br>bytes 1-5 - first element of derivation path in big endian format<br>bytes 6-9 - second element of derivation path in big endian format<br>... - remaining elements of derivation path |

Response (32 bytes):

| Data      | Description        |
|-----------|--------------------|
| byte 0-31 | Ed25519 public key |

## GetPubKeySecp256k1

Get Secp256k1 public key for provided derivation path.

APDU:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                 |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x31 | 0x00 | 0x00 | Derivation path in the following format:<br>byte 0 - number of elements in derivation path<br>bytes 1-5 - first element of derivation path in big endian format<br>bytes 6-9 - second element of derivation path in big endian format<br>... - remaining elements of derivation path |

Response (33 bytes):

| Data      | Description          |
|-----------|----------------------|
| byte 0-32 | Secp256k1 public key |

Returned key is a compressed key. First byte is always 0x02 or 0x03, depending on the parity of the y-coordinate.
The remaining 32 bytes are the x-coordinate.

## SignTxEd25519

Sign transaction intent using Ed25519 private key and derivation path. Derivation path should follow the format
described in CAP-26 SLIP 10 HD Derivation Path Scheme.

This command decodes transaction intent, retrieves instructions with their parameters and shows them to the user. Since
decoded instruction and parameters may exceed available device resources, the information shown to user might be
incomplete.

This command is invoked in two steps:

- Send derivation path.
- Send transaction intent data (see below). This command can be sent one or more times, depending on the size of the
  transaction intent. The last chunk is accompanied with class byte set to `0xAC`. Other (intermediate) chunks are
  accompanied with class byte set to `0xAD`.

APDU for derivation path:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                               |
|------|------|------|------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x41 | 0x00 | 0x00 | Payload should contain derivation path in the following format:<br>byte 0 - number of elements in derivation path bytes 1-5 - first element of derivation path in big endian format bytes 6-9 - second element of derivation path in big endian format ... - remaining elements of derivation path |

APDU for transaction intent data:

| CLA                                                     | INS  | P1   | P2   | Data                          |
|---------------------------------------------------------|------|------|------|-------------------------------|
| 0xAD - for intermediate chunks<br>0xAC - for last chunk | 0x41 | 0x00 | 0x00 | Transaction intent data chunk |

Upon successful sign, the device returns the signature for the transaction intent. The signature is returned in the
following format:

| Data        | Description        |
|-------------|--------------------|
| byte 0-63   | Ed25519 signature  |
| byte 64-95  | Ed25519 public key |
| byte 96-127 | Calculated digest  |

If user rejects the sign request, then the device returns error code 0x6e50 (User rejected the sign request).

## SignTxSecp256k1

Sign transaction intent using Secp256k1 private key and derivation path.

This command decodes transaction intent, retrieves instructions with their parameters and shows them to the user. Since
decoded instruction and parameters may exceed available device resources, the information shown to user might be
incomplete.

This command is invoked in two steps:

- Send derivation path.
- Send transaction intent data (see below). This command can be sent one or more times, depending on the size of the
  transaction intent. The last chunk is accompanied with class byte set to `0xAC`. Other (intermediate) chunks are
  accompanied with class byte set to `0xAD`.

APDU for derivation path:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                 |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x51 | 0x00 | 0x00 | Derivation path in the following format:<br>byte 0 - number of elements in derivation path<br>bytes 1-5 - first element of derivation path in big endian format<br>bytes 6-9 - second element of derivation path in big endian format<br>... - remaining elements of derivation path |

APDU for transaction intent data:

| CLA                                                     | INS  | P1   | P2   | Data                          |
|---------------------------------------------------------|------|------|------|-------------------------------|
| 0xAD - for intermediate chunks<br>0xAC - for last chunk | 0x41 | 0x00 | 0x00 | Transaction intent data chunk |

Upon successful sign, the device returns the signature for the transaction intent. The signature is returned in the
following format:

| Data        | Description          |
|-------------|----------------------|
| byte 0-64   | Secp256k1 signature  |
| byte 65-97  | Secp256k1 public key |
| byte 98-130 | Calculated digest    |

If user rejects the sign request, then the device returns error code 0x6e50 (User rejected the sign request).

## SignAuthEd25519

Sign auth request data using Ed25519 private key and derivation path. Derivation path should follow the format described
in CAP-26 SLIP 10 HD Derivation Path Scheme.

This command is invoked in two steps:

- Send derivation path.
- Send auth request data (nonce, origin and dApp address). This packet uses class byte set to `0xAC`.

APDU for derivation path:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                 |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x61 | 0x00 | 0x00 | Derivation path in the following format:<br>byte 0 - number of elements in derivation path<br>bytes 1-5 - first element of derivation path in big endian format<br>bytes 6-9 - second element of derivation path in big endian format<br>... - remaining elements of derivation path |

APDU for auth request data:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                       |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAC | 0x61 | 0x00 | 0x00 | Auth request data in the following format:<br>byte 0-31 - nonce<br>byte 32 - dApp address length<br>byte 33-... - dApp address<br>byte ... - bytes left after dApp address contains origin |

Upon successful sign, the device returns the signature for the given auth request. The signature is returned in the
following format:

| Data        | Description        |
|-------------|--------------------|
| byte 0-63   | Ed25519 signature  |
| byte 64-95  | Ed25519 public key |
| byte 96-127 | Calculated digest  |

If user rejects the sign request, then the device returns error code 0x6e50 (User rejected the sign request).

## SignAuthSecp256k1

Sign auth request data using Secp256k1 private key and derivation path.

This command is invoked in two steps:

- Send derivation path. The request format is the same as for GetPubKeyEd25519 command except different instruction
  code (see below).
- Send auth request data (nonce, origin and dApp address). This packet uses class byte set to `0xAC`.

APDU for derivation path:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                 |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x71 | 0x00 | 0x00 | Derivation path in the following format:<br>byte 0 - number of elements in derivation path<br>bytes 1-5 - first element of derivation path in big endian format<br>bytes 6-9 - second element of derivation path in big endian format<br>... - remaining elements of derivation path |

APDU for auth request data:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                       |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAC | 0x71 | 0x00 | 0x00 | Auth request data in the following format:<br>byte 0-31 - nonce<br>byte 32 - dApp address length<br>byte 33-... - dApp address<br>byte ... - bytes left after dApp address contains origin |

Upon successful sign, the device returns the signature for the given auth request. The signature is returned in the
following format:

| Data        | Description          |
|-------------|----------------------|
| byte 0-64   | Secp256k1 signature  |
| byte 65-97  | Secp256k1 public key |
| byte 98-130 | Calculated digest    |

If user rejects the sign request, then the device returns error code 0x6e50 (User rejected the sign request).

## VerifyAddressEd25519

Verify bech32m address for a given derivation path using Ed25519 curve.

APDU:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                 |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x81 | 0x00 | 0x00 | Derivation path in the following format:<br>byte 0 - number of elements in derivation path<br>bytes 1-5 - first element of derivation path in big endian format<br>bytes 6-9 - second element of derivation path in big endian format<br>... - remaining elements of derivation path |

Response:

| Data       | Description                              |
|------------|------------------------------------------|
| byte 0-... | bech32m address calculated by the device |    

## VerifyAddressSecp256k1

Verify bech32m address for a given derivation path using Secp256k1 curve.

APDU:

| CLA  | INS  | P1   | P2   | Data                                                                                                                                                                                                                                                                                 |
|------|------|------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0xAA | 0x91 | 0x00 | 0x00 | Derivation path in the following format:<br>byte 0 - number of elements in derivation path<br>bytes 1-5 - first element of derivation path in big endian format<br>bytes 6-9 - second element of derivation path in big endian format<br>... - remaining elements of derivation path |

Response:

| Data       | Description                              |
|------------|------------------------------------------|
| byte 0-... | bech32m address calculated by the device |    


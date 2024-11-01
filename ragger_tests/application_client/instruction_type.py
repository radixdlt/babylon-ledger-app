
from enum import IntEnum

class InsType(IntEnum):
    GET_APP_VERSION                 = 0x10
    GET_DEVICE_MODEL                = 0x11
    GET_DEVICE_ID                   = 0x12
    GET_APP_SETTINGS                = 0x22
    GET_PUB_KEY_ED25519             = 0x21
    GET_PUB_KEY_SECP256K1           = 0x31
    SIGN_TX_ED25519                 = 0x41
    SIGN_TX_SECP256K1               = 0x51
    SIGN_AUTH_ED25519               = 0x61
    SIGN_AUTH_SECP256K1             = 0x71
    VERIFY_ADDRESS_ED25519          = 0x81
    VERIFY_ADDRESS_SECP256K1        = 0x91
    SIGN_PRE_AUTH_HASH_ED25519      = 0xA1
    SIGN_PRE_AUTH_HASH_SECP256K1    = 0xA2
    SIGN_PRE_AUTH_RAW_ED25519       = 0xA3
    SIGN_PRE_AUTH_RAW_SECP256K1     = 0xA4
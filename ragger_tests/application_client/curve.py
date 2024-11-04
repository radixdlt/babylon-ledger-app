from abc import ABC, abstractmethod
from typing import TypeVar

from application_client.instruction_type import InsType
from response_unpacker import PK, Curve25519PublicKey, SignedPayloadEd25519, SignedPayloadSecp256k1, Signed, Secp256k1PublicKey

class Curve(ABC):
   
    @classmethod
    @abstractmethod
    def curve_name(cls) -> str:
        pass

    @classmethod
    @abstractmethod
    def unpack_pubkey(cls, response: bytes) -> PK:
        pass

    @classmethod
    @abstractmethod
    def unpack_signed(cls, response: bytes) -> Signed:
        pass

    @classmethod
    @abstractmethod
    def ins_sign_rola(cls) -> InsType:
        pass

    @classmethod
    @abstractmethod
    def ins_sign_tx(cls) -> InsType:
        pass

    @classmethod
    @abstractmethod
    def ins_get_pubkey(cls) -> InsType:
        pass

    @classmethod
    @abstractmethod
    def ins_verify_address(cls) -> InsType:
        pass

    @classmethod
    @abstractmethod
    def ins_sign_pre_auth_hash(cls) -> InsType:
        pass

    @classmethod
    @abstractmethod
    def ins_sign_pre_auth_raw(cls) -> InsType:
        pass

class SECP256K1(Curve):

    @classmethod
    def curve_name(cls) -> str:
        return "secp256k1"
    
    @classmethod
    def ins_sign_rola(cls) -> InsType:
        return InsType.SIGN_AUTH_SECP256K1

    @classmethod
    def ins_sign_tx(cls) -> InsType:
        return InsType.SIGN_TX_SECP256K1

    @classmethod
    def ins_get_pubkey(cls) -> InsType:
        return InsType.GET_PUB_KEY_SECP256K1

    @classmethod
    def ins_verify_address(cls) -> InsType:
        return InsType.VERIFY_ADDRESS_SECP256K1

    @classmethod
    def ins_sign_pre_auth_hash(cls) -> InsType:
        return InsType.SIGN_PRE_AUTH_HASH_SECP256K1

    @classmethod
    def ins_sign_pre_auth_raw(cls) -> InsType:
        return InsType.SIGN_PRE_AUTH_RAW_SECP256K1
    
    @classmethod
    def unpack_signed(cls, response: bytes) -> SignedPayloadSecp256k1:
        return SignedPayloadSecp256k1.unpack_response(response)

    @classmethod
    def unpack_pubkey(cls, response: bytes) -> PK:
        return Secp256k1PublicKey.unpack(response)


class Curve25519(Curve):

    @classmethod
    def curve_name(cls) -> str:
        return "ed25519"
    
    @classmethod
    def ins_sign_rola(cls) -> InsType:
        return InsType.SIGN_AUTH_ED25519

    @classmethod
    def ins_sign_tx(cls) -> InsType:
        return InsType.SIGN_TX_ED25519

    @classmethod
    def ins_get_pubkey(cls) -> InsType:
        return InsType.GET_PUB_KEY_ED25519

    @classmethod
    def ins_verify_address(cls) -> InsType:
        return InsType.VERIFY_ADDRESS_ED25519

    @classmethod
    def ins_sign_pre_auth_hash(cls) -> InsType:
        return InsType.SIGN_PRE_AUTH_HASH_ED25519

    @classmethod
    def ins_sign_pre_auth_raw(cls) -> InsType:
        return InsType.SIGN_PRE_AUTH_RAW_ED25519
    
    @classmethod
    def unpack_signed(cls, response: bytes) -> SignedPayloadEd25519:
        return SignedPayloadEd25519.unpack_response(response)
    
    @classmethod
    def unpack_pubkey(cls, response: bytes) -> PK:
        return Curve25519PublicKey.unpack(response)

C = TypeVar('C', bound=Curve)
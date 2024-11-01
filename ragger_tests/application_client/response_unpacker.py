from __future__ import annotations
from dataclasses import dataclass
from enum import IntEnum
from typing import NamedTuple, Tuple, TypeVar
from struct import unpack
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey
from cryptography.hazmat.primitives.asymmetric.ec import EllipticCurvePublicKey, ECDSA, SECP256K1
from cryptography.hazmat.primitives.asymmetric import ec, utils
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives._serialization import Encoding, PublicFormat
from abc import ABC, abstractmethod


PK = TypeVar('PK', bound="IsPublicKey")

class IsPublicKey(ABC):
    @abstractmethod
    def verify_signature(self, signature: bytes, hash: bytes) -> bool:
        pass

    @abstractmethod
    def serialize(self) -> bytes:
        pass

    @classmethod
    @abstractmethod
    def unpack(cls, raw: bytes) -> PK:
        pass

@dataclass
class Curve25519PublicKey(IsPublicKey):
    wrapped_key: Ed25519PublicKey

    def serialize(self) -> bytes:
        return self.wrapped_key.public_bytes_raw()

    def verify_signature(self, signature: bytes, hash: bytes) -> bool:
        try:
            self.wrapped_key.verify(signature=signature, data=hash)
            return True
        except Exception as e:
            raise ValueError("Invalid signature: ", e)
        
    @classmethod
    def unpack(cls, raw: bytes) -> Curve25519PublicKey:
        wrapped_key = Ed25519PublicKey.from_public_bytes(data=raw)
        return Curve25519PublicKey(wrapped_key=wrapped_key)
        

@dataclass
class Secp256k1PublicKey(IsPublicKey):
    wrapped_key: EllipticCurvePublicKey

    def serialize(self) -> bytes:
        return self.wrapped_key.public_bytes(Encoding.X962, PublicFormat.CompressedPoint)

    def verify_signature(self, signature: bytes, hash: bytes) -> bool:
        try:
            self.wrapped_key.verify(
                signature, 
                hash, 
                ec.ECDSA(utils.Prehashed(hashes.SHA256()))
            )
            return True
        except Exception as e:
            raise ValueError("Invalid signature: ", e)
        
    @classmethod
    def unpack(cls, raw: bytes) -> Secp256k1PublicKey:
        wrapped_key = ec.EllipticCurvePublicKey.from_encoded_point(curve=SECP256K1(), data=raw)
        return Secp256k1PublicKey(wrapped_key=wrapped_key)


class Version(NamedTuple):
    major: int
    minor: int
    patch: int

@dataclass
class ROLAResponse(ABC):
    key: PK
    hash: bytes
    signature: bytes

    def verify_signature(self) -> bool:
        return self.key.verify_signature(signature=self.signature, hash=self.hash)
    
    @classmethod
    @abstractmethod
    def unpack_key(cls, raw: bytes) -> PK:
        pass

    @classmethod
    @abstractmethod
    def unpack_signature(cls, raw: bytes) -> bytes:
        pass

    @classmethod
    @abstractmethod
    def sig_len(cls) -> int:
        pass

    @classmethod
    @abstractmethod
    def key_len(cls) -> int:
        pass

    @classmethod
    def hash_len(cls) -> int:
        return 32 

    @classmethod
    @abstractmethod
    def expected_len(cls) -> int:
        pass

    @classmethod
    @abstractmethod
    def unpack_response(cls, response: bytes) -> ROLAResp:
        pass

    @classmethod
    def raw_unpack_response(cls, response: bytes) -> Tuple[PK, bytes, bytes]:
        expected_byte_count = cls.expected_len()
        byte_count = len(response)
        if byte_count != expected_byte_count:
            raise ValueError(f"Invalid length, expected #{expected_byte_count} bytes, got: #{byte_count}")
        signature_bytes = response[0:cls.sig_len()]
        key_start = cls.sig_len()
        key_end = key_start + cls.key_len()
        keybytes = response[key_start:key_end]
        hash_start = key_end
        hash_end = hash_start + cls.hash_len()
        hash = response[hash_start:hash_end]
        assert len(signature_bytes) == cls.sig_len()
        assert len(keybytes) == cls.key_len()
        assert len(hash) == cls.hash_len()

        signature = cls.unpack_signature(raw=signature_bytes)
        key = cls.unpack_key(keybytes)

        return key, hash, signature

class ROLAResponseEd25519(ROLAResponse):
    key: Curve25519PublicKey

    @classmethod
    def unpack_key(cls, raw: bytes) -> Curve25519PublicKey:
        return Curve25519PublicKey.unpack(raw=raw)
    
    @classmethod
    def unpack_signature(cls, raw: bytes) -> bytes:
        return raw # no parsing needed

    @classmethod
    def sig_len(cls) -> int:
        return 64

    @classmethod
    def key_len(cls) -> int:
        return 32

    @classmethod
    def expected_len(cls) -> int:
        return cls.sig_len() + cls.key_len() + cls.hash_len()

    @classmethod
    def unpack_response(cls, response: bytes) -> ROLAResp:
        (key, hash, signature) = cls.raw_unpack_response(response)
        return ROLAResponseEd25519(key=key, hash=hash, signature=signature)
        
class ROLAResponseSecp256k1(ROLAResponse):
    key: EllipticCurvePublicKey

    @classmethod
    def unpack_key(cls, raw: bytes) -> Secp256k1PublicKey:
        return Secp256k1PublicKey.unpack(raw=raw)
    
    @classmethod
    def unpack_signature(cls, raw: bytes) -> bytes:
        if len(raw) != cls.sig_len():
            raise ValueError("Invalid signature length")
        r = int.from_bytes(raw[1:33], byteorder='big', signed=False)
        s = int.from_bytes(raw[33:65], byteorder='big', signed=False)
        return utils.encode_dss_signature(r, s)

    @classmethod
    def unpack_response(cls, response: bytes) -> ROLAResp:
        (key, hash, signature) = cls.raw_unpack_response(response)
        return ROLAResponseSecp256k1(key=key, hash=hash, signature=signature)

    @classmethod
    def expected_len(cls) -> int:
        return cls.sig_len() + cls.key_len() + cls.hash_len()

    @classmethod
    def sig_len(cls) -> int:
        return 65

    @classmethod
    def key_len(cls) -> int:
        return 33

ROLAResp = TypeVar('ROLAResp', bound=ROLAResponse)

def unpack_get_version_response(response: bytes) -> Version:
    assert len(response) == 3
    major, minor, patch = unpack("BBB", response)
    return Version(major=major, minor=minor, patch=patch)

class LedgerModel(IntEnum):
    NANO_S      = 0x00
    NANO_S_PLUS = 0x01
    NANO_X      = 0x02
    STAX        = 0x04

    def unpack(raw: bytes) -> LedgerModel:
        int_val = int.from_bytes(raw, byteorder='big', signed=False)
        return LedgerModel(int_val)

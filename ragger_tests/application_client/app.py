import hashlib
from typing import Generator, Callable, Optional
from contextlib import contextmanager

from ragger.backend.interface import BackendInterface, RAPDU
from response_unpacker import PK, LedgerAppSettings, LedgerModel, Version, SignedPayloadEd25519, SignedPayloadSecp256k1, Signed, unpack_get_version_response
from command_sender import CommandSender, InsType
from request_packer import RequestPacker
from application_client.curve import C

class App:
    def __init__(self, backend: BackendInterface) -> None:
        self.sender = CommandSender(backend)
    
    def access_last_async_response(self) -> Optional[RAPDU]:
        return self.sender.get_optional_async_response()

    def get_version(self) -> Version:
        rapdu = self.sender.get_version()
        return unpack_get_version_response(rapdu.data)

    def get_device_id(self) -> str:
        rapdu = self.sender.get_device_id()
        return rapdu.data.hex()
    
    def get_app_settings(self) -> LedgerAppSettings:
        rapdu = self.sender.get_app_settings()
        return LedgerAppSettings.unpack(raw=rapdu.data)
    
    def get_device_model(self) -> LedgerModel:
        rapdu = self.sender.get_device_model()
        return LedgerModel.unpack(raw=rapdu.data)
    
    @contextmanager
    def __verify_address(
        self, 
        curve: C,
        navigate: Callable[[], None],
        path: str, 
    ) -> Generator[RAPDU, None, None]:
        global maybe
        with self.sender.send_verify_address(
            curve=curve,
            navigate=navigate,
            path=path,
        ) as response:
            maybe = response
        yield maybe if maybe is not None else self.sender.get_async_response()

    def verify_address(
        self, 
        curve: C,
        path: str, 
        navigate: Callable[[], None] = lambda: None
    ) -> str:
        global response
        with self.__verify_address(
            curve=curve,
            navigate=navigate,
            path=path
        ) as res:
            response = res
        return response.data.decode('utf-8')

    @contextmanager
    def __sign_generic(
        self, 
        ins: InsType,
        navigate_path: Callable[[], None],
        navigate_sign: Callable[[], None],
        path: str, 
        payload: bytes
    ) -> Generator[RAPDU, None, None]:
        global maybe
        with self.sender.send_sign_generic(
            ins,
            navigate_path=navigate_path,
            navigate_sign=navigate_sign,
            path=path,
            payload=payload
        ) as response:
            maybe = response
        yield maybe if maybe is not None else self.sender.get_async_response()

    def sign_rola(
        self, 
        curve: C,
        path: str, 
        dapp_def_addr: str, 
        origin: str, 
        nonce: bytes,
        navigate_path: Callable[[], None] = lambda: None,
        navigate_sign: Callable[[], None] = lambda: None
    ) -> Signed:
        """
        Forms a ROLA Challenge from (`dapp_def_addr`, `origin`, `nonce`) and signs it
        using `path`. This will send two APDU requests to the Ledger, one first with
        the derivation path, which requires interaction on device, before host (this Python app)
        will send the next APDU request with the ROLA challenge.

        :param path: The BIP32 derivation path as a string.
        :type path: str
        :param dapp_def_addr: The dapp definition address, used to form ROLA challenge.
        :type dapp_def_addr: str
        :param origin: The origin of the Dapp sending the auth request, used to form ROLA challenge.
        :type origin: str
        :param nonce: The nonce sent by the Dapp sending the auth request, used to form ROLA challenge.
        :type nonce: bytes
        :param navigate_path: A closure passed to proceed from having sent the first APDU request. **Pass an empty closure (`lambda: None`) if you are using a physical device**.
        :type navigate_path: Optional closure (`Callable`). 
        :param navigate_sign: A closure passed to confirm signing of the ROLA challenge, use `navigator.navigate_and_compare` to pass in interactions. **Pass an empty closure (`lambda: None`) if you are using a physical device** 
        :type navigate_sign: Optional closure (`Callable`)

        :return: A signed payload parsed from the APDU response containing `(hash, signature, public_key)`
        :rtype: Signed
        """
        rola_challenge = RequestPacker.pack_rola_request(
            dapp_def_addr=dapp_def_addr, 
            origin=origin, 
            nonce=nonce
        )
        global response
        with self.__sign_generic(
            ins=curve.ins_sign_rola(),
            navigate_path=navigate_path,
            navigate_sign=navigate_sign,
            path=path,
            payload=rola_challenge
        ) as res:
            response = res
        return curve.unpack_signed(response=response.data)

    def sign_tx(
        self, 
        curve: C,
        path: str, 
        txn: bytes,
        navigate_path: Callable[[], None] = lambda: None,
        navigate_sign: Callable[[], None] = lambda: None
    ) -> Signed:
        global response
        with self.__sign_generic(
            ins=curve.ins_sign_tx(),
            navigate_path=navigate_path,
            navigate_sign=navigate_sign,
            path=path,
            payload=txn
        ) as res:
            response = res
        return curve.unpack_signed(response=response.data)
    
    def sign_preauth_raw(
        self, 
        curve: C,
        path: str, 
        txn: bytes,
        navigate_path: Callable[[], None] = lambda: None,
        navigate_sign: Callable[[], None] = lambda: None
    ) -> Signed:
        global response
        with self.__sign_generic(
            ins=curve.ins_sign_pre_auth_raw(),
            navigate_path=navigate_path,
            navigate_sign=navigate_sign,
            path=path,
            payload=txn
        ) as res:
            response = res
        return curve.unpack_signed(response=response.data)
    
    def sign_preauth_hash(
        self, 
        curve: C,
        path: str, 
        message_to_hash: bytes,
        navigate_path: Callable[[], None] = lambda: None,
        navigate_sign: Callable[[], None] = lambda: None
    ) -> Signed:
        hash_calculator = hashlib.blake2b(digest_size=32)
        hash_calculator.update(message_to_hash)
        hash = hash_calculator.digest()
        global response
        with self.__sign_generic(
            ins=curve.ins_sign_pre_auth_hash(),
            navigate_path=navigate_path,
            navigate_sign=navigate_sign,
            path=path,
            payload=hash
        ) as res:
            response = res
        return curve.unpack_signed(response=response.data)

    def get_public_key(
        self, 
        curve: C,
        path: str, 
    ) -> PK:
        """
        Get the public key of the given derivation path.

        :param path: The BIP32 derivation path as a string.
        :type path: str

        :return: The public key of the given derivation path.
        :rtype: PK
        """
        response = self.sender.get_public_key(
            curve=curve,
            path=path
        )
        return curve.unpack_pubkey(response.data)
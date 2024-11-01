from typing import Generator, Callable, Optional
from contextlib import contextmanager

from ragger.backend.interface import BackendInterface, RAPDU
from response_unpacker import PK, LedgerModel, Version, ROLAResponseEd25519, ROLAResponseSecp256k1, ROLAResp, unpack_get_version_response
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
    
    def get_device_model(self) -> LedgerModel:
        rapdu = self.sender.get_device_model()
        return LedgerModel.unpack(raw=rapdu.data)

    @contextmanager
    def __sign_rola(
        self, 
        curve: C,
        navigate_path: Callable[[], None],
        navigate_sign: Callable[[], None],
        path: str, 
        dapp_def_addr: str, 
        origin: str, 
        nonce: bytes
    ) -> Generator[RAPDU, None, None]:
        rola_challenge = RequestPacker.pack_rola_request(
            dapp_def_addr=dapp_def_addr, 
            origin=origin, 
            nonce=nonce
        )
        global maybe
        with self.sender.send_sign_auth(
            curve=curve,
            navigate_path=navigate_path,
            navigate_sign=navigate_sign,
            path=path,
            rola_challenge=rola_challenge
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
    ) -> ROLAResp:
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

        :return: The a ROLAEd25519Response parse from the APDU response.
        :rtype: ROLAEd25519Response
        """
        global response
        with self.__sign_rola(
            curve=curve,
            navigate_path=navigate_path,
            navigate_sign=navigate_sign,
            path=path,
            dapp_def_addr=dapp_def_addr,
            origin=origin,
            nonce=nonce
        ) as res:
            response = res
        return curve.unpack_rola_response(response=response.data)

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
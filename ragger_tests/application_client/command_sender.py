from enum import IntEnum
from typing import Generator, Callable, Optional
from contextlib import contextmanager

from ragger.backend.interface import BackendInterface, RAPDU
from ragger.bip import pack_derivation_path
from application_client.instruction_type import InsType
from application_client.curve import C

MAX_APDU_LEN: int = 255

CLA: int = 0xAA
CLA2: int = 0xAC

class P1(IntEnum):
    # Parameter 1 for first APDU number.
    START = 0x00

class P2(IntEnum):
    # Parameter 2 for last APDU to receive.
    LAST = 0x00

class CommandSender:
    def __init__(self, backend: BackendInterface) -> None:
        self.backend = backend

    def _send_ins(
        self,
        ins: InsType, 
        p1: P1 = P1.START, 
        p2: P2 = P2.LAST, 
        data: bytes = b"",
        cla: int = CLA
    ) -> RAPDU:
        print(f"🛰️ sending data (exchange): {data.hex()}")
        rc = self.backend.exchange(cla=cla, ins=ins, p1=p1, p2=p2, data=data)
        if rc is not None:
            print(f"🛰️ received data: {rc.data.hex()}")
        return rc
    
    @contextmanager
    def _async_send_ins(
        self,
        navigate: Callable[[], None],
        ins: InsType, 
        p1: P1 = P1.START, 
        p2: P2 = P2.LAST, 
        data: bytes = b"",
        cla: int = CLA
    ) -> Generator[None, None, None]:
        print(f"🛰️ sending data (exchange_async): {data.hex()}")
        with self.backend.exchange_async(cla=cla, ins=ins, p1=p1, p2=p2, data=data):
            navigate()
            yield None

    def get_optional_async_response(self) -> Optional[RAPDU]:
        return self.backend.last_async_response

    def get_async_response(self) -> RAPDU:
        opt = self.get_optional_async_response()
        if opt is None:
            raise ValueError("No response received")
        return opt 

    def get_version(self) -> RAPDU:
        return self._send_ins(
            ins=InsType.GET_APP_VERSION,
        )

    def get_device_id(self) -> RAPDU:
        return self._send_ins(
            ins=InsType.GET_DEVICE_ID,
        )

    def get_device_model(self) -> RAPDU:
        return self._send_ins(
            ins=InsType.GET_DEVICE_MODEL,
        )
    
    def get_public_key(
        self,
        curve: C,
        path: str
    ) -> RAPDU:
        return self._send_ins(
            ins=curve.ins_get_pubkey(),
            data=pack_derivation_path(path)
        )
    
    @contextmanager
    def send_verify_address(
        self,
        curve: C,
        navigate: Callable[[], None],
        path: str
    ):
        ins = curve.ins_verify_address()
        with self._async_send_ins(
            navigate=navigate, 
            ins=ins, 
            data=pack_derivation_path(path)
        ):
            yield self.get_optional_async_response()
      
    
    def _send_derivation_path(
        self, 
        navigate: Callable[[], None],
        ins: InsType, 
        path: str
    ):
        with self._async_send_ins(
            navigate=navigate, 
            ins=ins, 
            data=pack_derivation_path(path)
        ):
            pass            

    @contextmanager
    def send_sign_auth(
        self, 
        curve: C,
        navigate_path: Callable[[], None],
        navigate_sign: Callable[[], None],
        path: str, 
        rola_challenge: bytes
    ):
        ins = curve.ins_sign_rola()
        self._send_derivation_path(
            navigate=navigate_path,
            ins=ins,
            path=path
        )
        
        self.backend._last_async_response = None

        with self._async_send_ins(
            navigate=navigate_sign, 
            ins=ins, 
            data=rola_challenge,
            cla=CLA2 # Continuation
        ):
            yield self.get_optional_async_response()
      
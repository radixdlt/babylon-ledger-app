from typing import Generator
from pathlib import Path
from ragger.bip import pack_derivation_path
from ragger.navigator import NavInsID
from contextlib import contextmanager
from cryptography.hazmat.primitives.asymmetric import ed25519
import hashlib

ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

CLA1 = 0xAA
CLA2 = 0xAC
INS = 0xA1


def enable_blind_signing(navigator, test_name):
    print("Enable blind signing")
    navigator.navigate([NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK,  # Settings
                        NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK,  # Blind signing
                        NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK,  # Enable
                        NavInsID.LEFT_CLICK, NavInsID.LEFT_CLICK],  # Main screen
                       screen_change_before_first_instruction=False)


def send_derivation_path(backend, path, navigator):
    with backend.exchange_async(cla=CLA1, ins=INS, data=pack_derivation_path(path)) as response:
        navigator.navigate([NavInsID.RIGHT_CLICK])


@contextmanager
def send_preauth_hash_request(backend, vector) -> Generator[None, None, None]:
    hash_calculator = hashlib.blake2b(digest_size=32)
    hash_calculator.update(vector)
    data = hash_calculator.hexdigest()

    with backend.exchange_async(cla=CLA2, ins=INS, data=bytes.fromhex(data)) as response:
        yield response


def sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, vector):
    enable_blind_signing(navigator, test_name)
    send_derivation_path(backend, "m/44'/1022'/12'/525'/1460'/0'", navigator)

    with send_preauth_hash_request(backend, vector):
        if firmware.device.startswith("nano"):
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name,
                                           [NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK,
                                            NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK])

    rc = backend.last_async_response.data
    pubkey = ed25519.Ed25519PublicKey.from_public_bytes(bytes(rc[64:96]))
    try:
        pubkey.verify(bytes(rc[0:64]), bytes(rc[96:128]))
    except Exception as e:
        print("Invalid signature ", e)


def test_sign_preauth_hash_ed25519_0(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'0')

def test_sign_preauth_hash_ed25519_1(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'1')


def test_sign_preauth_hash_ed25519_2(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'2')


def test_sign_preauth_hash_ed25519_3(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'3')


def test_sign_preauth_hash_ed25519_4(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'4')


def test_sign_preauth_hash_ed25519_5(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'5')


def test_sign_preauth_hash_ed25519_6(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'6')


def test_sign_preauth_hash_ed25519_7(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'7')


def test_sign_preauth_hash_ed25519_8(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'8')


def test_sign_preauth_hash_ed25519_9(firmware, backend, navigator, test_name):
    sign_preauth_hash_ed25519(firmware, backend, navigator, test_name, b'9')

from typing import Generator
from pathlib import Path
from ragger.bip import pack_derivation_path
from ragger.navigator import NavInsID
from contextlib import contextmanager
from cryptography.hazmat.primitives.asymmetric import ec, utils
from cryptography.hazmat.primitives import hashes
import hashlib

DATA_PATH = str(Path(__file__).parent.joinpath("data").absolute()) + "/"
ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

CLA1 = 0xAA
CLA2 = 0xAC
INS = 0xA4

def read_file(file):
    with open(DATA_PATH + file, "rb") as f:
        return f.read()

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
def send_preauth_raw_request(backend, vector) -> Generator[None, None, None]:
    subintent = read_file(vector)
    num_chunks = len(subintent) // 255 + 1

    for i in range(num_chunks):
        chunk = subintent[i * 255:(i + 1) * 255]

        if i != num_chunks - 1:
            cls = 0xAB
            backend.exchange(cla=cls, ins=INS, p1=0, p2=0, data=chunk)
        else:
            cls = 0xAC
            with backend.exchange_async(cla=cls, ins=INS, p1=0, p2=0, data=chunk) as response:
                yield response


def sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, vector):
    enable_blind_signing(navigator, test_name)
    send_derivation_path(backend, "m/44'/1022'/10'/525'/1238'", navigator)

    with send_preauth_raw_request(backend, vector):
        if firmware.device.startswith("nano"):
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name,
                                           [NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK,
                                            NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK])

    rc = backend.last_async_response.data
    r = int.from_bytes(rc[1:33], byteorder='big', signed=False)
    s = int.from_bytes(rc[33:65], byteorder='big', signed=False)
    signature = utils.encode_dss_signature(int(r), int(s))
    pubkey = ec.EllipticCurvePublicKey.from_encoded_point(ec.SECP256K1(), bytes(rc[65:98]))
    try:
        # Note that Prehashed parameter is irrelevant here, we just need to pass something known to the library
        pubkey.verify(signature, bytes(rc[98:130]), ec.ECDSA(utils.Prehashed(hashes.SHA256())))
        print("Success")
    except Exception as e:
        print("Invalid signature ", e)


def test_sign_preauth_raw_secp256k1_0(firmware, backend, navigator, test_name):
    sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, 'subintent_vector_0.si')

def test_sign_preauth_raw_secp256k1_1(firmware, backend, navigator, test_name):
    sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, 'subintent_vector_1.si')


def test_sign_preauth_raw_secp256k1_2(firmware, backend, navigator, test_name):
    sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, 'subintent_vector_2.si')


def test_sign_preauth_raw_secp256k1_3(firmware, backend, navigator, test_name):
    sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, 'subintent_vector_3.si')


def test_sign_preauth_raw_secp256k1_4(firmware, backend, navigator, test_name):
    sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, 'subintent_vector_4.si')


def test_sign_preauth_raw_secp256k1_5(firmware, backend, navigator, test_name):
    sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, 'subintent_vector_5.si')


def test_sign_preauth_raw_secp256k1_6(firmware, backend, navigator, test_name):
    sign_preauth_raw_secp256k1(firmware, backend, navigator, test_name, 'subintent_vector_6.si')

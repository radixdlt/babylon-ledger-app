from ragger.backend.interface import BackendInterface
from ragger.firmware.structs import Firmware
from ragger.navigator.navigator import Navigator
from ragger_tests.application_client.curve import SECP256K1
from ragger_tests.test_sign_preauth_hash_ed25519 import sign_preauth_hash

def sign_preauth_hash_secp256k1(
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator,
    test_name: str, 
    message_to_hash: bytes
):
    sign_preauth_hash(
        curve=SECP256K1,
        path="m/44'/1022'/10'/525'/1238'",
        firmware=firmware,
        backend=backend,
        navigator=navigator,
        test_name=test_name,
        message_to_hash=message_to_hash
    )

def test_sign_preauth_hash_secp256k1_0(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'0')

def test_sign_preauth_hash_secp256k1_1(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'1')

def test_sign_preauth_hash_secp256k1_2(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'2')

def test_sign_preauth_hash_secp256k1_3(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'3')

def test_sign_preauth_hash_secp256k1_4(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'4')

def test_sign_preauth_hash_secp256k1_5(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'5')

def test_sign_preauth_hash_secp256k1_6(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'6')

def test_sign_preauth_hash_secp256k1_7(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'7')

def test_sign_preauth_hash_secp256k1_8(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'8')

def test_sign_preauth_hash_secp256k1_9(firmware, backend, navigator, test_name):
    sign_preauth_hash_secp256k1(firmware, backend, navigator, test_name, b'9')

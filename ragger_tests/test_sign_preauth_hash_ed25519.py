from pathlib import Path
from ragger.navigator import NavInsID
from ragger.navigator import NavInsID
from ragger.backend.interface import BackendInterface
from ragger.firmware.structs import Firmware
from ragger.navigator.navigator import Navigator
from ragger.backend.speculos import SpeculosBackend

from ragger_tests.application_client.app import App
from ragger_tests.application_client.curve import C, Curve25519
from ragger_tests.test_sign_tx_ed25519 import BlindSigningSettings

ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

def enable_blind_signing(navigator: Navigator):
    print("Enable blind signing")
    navigator.navigate(
        instructions=[
            NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK,  # Settings
            NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK,  # Blind signing
            NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK,  # Enable
            NavInsID.LEFT_CLICK, NavInsID.LEFT_CLICK    # Main screen
        ],  
        screen_change_before_first_instruction=False
    )

def sign_preauth_hash(
    curve: C,
    path: str,
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator,
    test_name: str, 
    message_to_hash: bytes
):
    if isinstance(backend, SpeculosBackend):
        enable_blind_signing(navigator)
    elif BlindSigningSettings.SKIP_IF_OFF.should_abort_execution_due_to_blind_sign(backend):
        return

    def navigate_path():
        navigator.navigate([NavInsID.RIGHT_CLICK])

    def navigate_sign():
        if firmware.is_nano:
            navigator.navigate_and_compare(
                path=ROOT_SCREENSHOT_PATH, 
                test_case_name=test_name,
                instructions=[
                    NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK,
                    NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK
                ]
            )

    app = App(backend)
    response = app.sign_preauth_hash(
        curve=curve,
        path=path, 
        message_to_hash=message_to_hash,
        navigate_path=navigate_path,
        navigate_sign=navigate_sign,
    )

    assert response.verify_signature()

def sign_preauth_hash_ed25519(
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator,
    test_name: str, 
    message_to_hash: bytes
):
    sign_preauth_hash(
        curve=Curve25519,
        path="m/44'/1022'/12'/525'/1460'/0'",
        firmware=firmware,
        backend=backend,
        navigator=navigator,
        test_name=test_name,
        message_to_hash=message_to_hash
    )

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

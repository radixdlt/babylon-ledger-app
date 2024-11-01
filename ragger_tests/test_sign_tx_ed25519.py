from enum import Enum
from pathlib import Path
from typing import List, Optional
from ragger.navigator import NavInsID
from ragger.backend.interface import BackendInterface
from ragger.backend.speculos import SpeculosBackend
from ragger.firmware.structs import Firmware
from ragger.navigator.navigator import Navigator

from ragger_tests.application_client.app import App
from ragger_tests.application_client.curve import C, Curve25519

DATA_PATH = str(Path(__file__).parent.joinpath("data").absolute()) + "/"
ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

class BlindSigningSettings(Enum):
    DONT_CHECK_SETTINGS = "DONT_CHECK_SETTINGS"
    FAIL_IF_OFF         = "FAIL_IF_OFF"
    FAIL_IF_ON          = "FAIL_IF_ON"
    SKIP_IF_OFF         = "SKIP_IF_OFF"
    SKIP_IF_ON          = "SKIP_IF_ON"

    def should_check(self) -> bool:
        return self != BlindSigningSettings.DONT_CHECK_SETTINGS

    def should_be_off(self) -> bool:
        return self == BlindSigningSettings.SKIP_IF_ON or self == BlindSigningSettings.FAIL_IF_ON

    def should_be_on(self) -> bool:
        return self == BlindSigningSettings.SKIP_IF_OFF or self == BlindSigningSettings.FAIL_IF_OFF

    def should_fail(self) -> bool:
        return self == BlindSigningSettings.FAIL_IF_OFF or self == BlindSigningSettings.FAIL_IF_ON

    def should_skip(self) -> bool:
        return self == BlindSigningSettings.SKIP_IF_ON or self == BlindSigningSettings.SKIP_IF_OFF

def sign_tx(
    curve: C,
    path: str,
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator, 
    click_count: int, 
    txn: bytes, 
    test_name: str,
    blind_signing_settings: BlindSigningSettings
):
    clicks: List[NavInsID] = []

    if click_count > 0:
        clicks = [NavInsID.RIGHT_CLICK] * click_count
        clicks.append(NavInsID.BOTH_CLICK)
    else:
        clicks = [NavInsID.RIGHT_CLICK]

    def navigate_path():
        navigator.navigate([NavInsID.RIGHT_CLICK])

    def navigate_sign():
        if firmware.device.startswith("nano"):
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name, clicks)
    
    app = App(backend)

    if blind_signing_settings.should_check() and not isinstance(backend, SpeculosBackend):
        app_settings = app.get_app_settings()
        is_blind_signing_enabled = app_settings.is_blind_signing_enabled
        if is_blind_signing_enabled and blind_signing_settings.should_be_off():
            errmsg = "⚙️ ❌ Blind signing is on, but required to be off."
            print(errmsg)
            if blind_signing_settings.should_fail():
                raise ValueError(errmsg)
            elif blind_signing_settings.should_skip():
                print("🙅‍♀️ Skipping test")
                return
        elif not is_blind_signing_enabled and blind_signing_settings.should_be_on():
            errmsg = "⚙️ ❌ Blind signing is off, but required to be on."
            print(errmsg)
            if blind_signing_settings.should_fail():
                raise ValueError(errmsg)
            elif blind_signing_settings.should_skip():
                print("🙅‍♀️ Skipping test")
                return
        else:
            enabled_or_not = "ENABLED" if is_blind_signing_enabled else "DISABLED"
            print(f"✅ Blind signing is: {enabled_or_not} which it was required to be.")

    response = app.sign_tx(
        curve=curve,
        path=path, 
        txn=txn,
        navigate_path=navigate_path,
        navigate_sign=navigate_sign,
    )

    assert response.verify_signature()

def read_file(file: str) -> bytes:
    with open(DATA_PATH + file, "rb") as f:
        return f.read()


def sign_tx_with_file_name(
    curve: C,
    path: str,
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator, 
    click_count: int, 
    file_name: str, 
    test_name: str,
    blind_signing_settings: BlindSigningSettings
):
    txn = read_file(file=file_name)
    sign_tx(
        curve=curve,
        path=path,
        firmware=firmware, 
        backend=backend,
        navigator=navigator,
        click_count=click_count,
        txn=txn,
        test_name=test_name,
        blind_signing_settings=blind_signing_settings
    )

def sign_tx_ed25519(
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator, 
    click_count: int, 
    file_name: str, 
    test_name: str,
    blind_signing_settings: BlindSigningSettings = BlindSigningSettings.DONT_CHECK_SETTINGS
):
    sign_tx_with_file_name(
        curve=Curve25519,
        path="m/44'/1022'/12'/525'/1460'/0'",
        firmware=firmware, 
        backend=backend,
        navigator=navigator,
        click_count=click_count,
        file_name=file_name,
        test_name=test_name,
        blind_signing_settings=blind_signing_settings
    )

# def test_sign_tx_ed25519_call_function(firmware, backend, navigator, test_name):
#     sign_tx_ed25519(
#         firmware, backend, navigator, 0, "call_function.txn", test_name, 
#         blind_signing_settings=BlindSigningSettings.SKIP_IF_OFF
#     )

def test_sign_tx_ed25519_simple_transfer(firmware, backend, navigator, test_name):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer.txn", test_name)

def test_sign_tx_ed25519_simple_transfer_new_format(firmware, backend, navigator, test_name):
    sign_tx_ed25519(firmware, backend, navigator, 10, "simple_transfer_new_format.txn", test_name)

def test_sign_tx_ed25519_simple_transfer_nft(firmware, backend, navigator, test_name):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft.txn", test_name)

def test_sign_tx_ed25519_simple_transfer_nft_by_id(firmware, backend, navigator, test_name):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft_by_id.txn", test_name)

def test_sign_tx_ed25519_simple_transfer_nft_new_format(firmware, backend, navigator, test_name):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft_new_format.txn", test_name)

def test_sign_tx_ed25519_simple_transfer_nft_by_id_new_format(firmware, backend, navigator, test_name):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft_by_id_new_format.txn", test_name)

def test_sign_tx_ed25519_simple_transfer_with_multiple_locked_fees(firmware, backend, navigator, test_name):
    sign_tx_ed25519(firmware, backend, navigator, 10, "simple_transfer_with_multiple_locked_fees.txn", test_name)

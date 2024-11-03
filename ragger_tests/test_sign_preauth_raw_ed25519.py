import os
from pathlib import Path
from ragger.backend.interface import BackendInterface
from ragger.firmware.structs import Firmware
from ragger.navigator.navigator import Navigator
from ragger.navigator import NavInsID

from ragger_tests.application_client.app import App
from ragger_tests.application_client.curve import C, Curve25519
from ragger_tests.test_sign_tx_ed25519 import BlindSigningSettings

DATA_PATH = str(Path(__file__).parent.joinpath("data").absolute()) + "/"
ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()


def sign_preauth_raw(
    curve: C,
    path: str,
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator, 
    txn: bytes, 
    test_name: str,
    blind_signing_settings: BlindSigningSettings
):

    def navigate_path():
        navigator.navigate_and_compare(
            path=ROOT_SCREENSHOT_PATH, 
            test_case_name=test_name,
            instructions=[
                NavInsID.RIGHT_CLICK
            ], 
            screen_change_before_first_instruction=True,
            snap_start_idx=0
        )

    def navigate_sign():
        if firmware.is_nano:
            navigator.navigate_and_compare(
                path=ROOT_SCREENSHOT_PATH, 
                test_case_name=test_name,
                instructions=[
                    NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK,
                    NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK
                ],
                screen_change_before_first_instruction=True,
                snap_start_idx=1
            )
    
    # if blind_signing_settings.should_abort_execution_due_to_blind_sign(backend):
    #     return

    app = App(backend)
    response = app.sign_preauth_raw(
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


def sign_preauth_raw_with_file_name(
    curve: C,
    path: str,
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator, 
    file_name: str, 
    test_name: str,
    blind_signing_settings: BlindSigningSettings
):
    print(f"🎃 file_name: '{file_name}'")
    txn = read_file(file=file_name)
    sign_preauth_raw(
        curve=curve,
        path=path,
        firmware=firmware, 
        backend=backend,
        navigator=navigator,
        txn=txn,
        test_name=test_name,
        blind_signing_settings=blind_signing_settings
    )

def sign_preauth_raw_ed25519(
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator, 
    file_name: str, 
    test_name: str,
    blind_signing_settings: BlindSigningSettings = BlindSigningSettings.DONT_CHECK_SETTINGS
):
    sign_preauth_raw_with_file_name(
        curve=Curve25519,
        path="m/44'/1022'/12'/525'/1460'/0'",
        firmware=firmware, 
        backend=backend,
        navigator=navigator,
        file_name=file_name,
        test_name=test_name,
        blind_signing_settings=blind_signing_settings
    )

def list_files():
    dir_path = "data"
    res = []
    for path in os.listdir(dir_path):
        if os.path.isfile(os.path.join(dir_path, path)):
            res.append(os.path.join(path))
    return res

def test_sign_preauth_raw_ed25519_all(firmware, backend, navigator):
    for file_name in list_files():
        if not file_name.endswith(".si"):
            continue
        sign_preauth_raw_ed25519(firmware, backend, navigator, file_name, test_name=file_name)



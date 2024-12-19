from ragger.backend.interface import BackendInterface
from ragger.firmware.structs import Firmware
from ragger.navigator.navigator import Navigator
from ragger.backend.speculos import SpeculosBackend

from ragger_tests.application_client.curve import SECP256K1
from ragger_tests.test_sign_preauth_hash_ed25519 import enable_blind_signing
from ragger_tests.test_sign_tx_ed25519 import BlindSigningSettings, sign_tx_with_file_name

def sign_tx_secp256k1(
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator, 
    click_count: int, 
    file_name: str, 
    test_name: str,
    blind_signing_settings: BlindSigningSettings = BlindSigningSettings.DONT_CHECK_SETTINGS
):
    sign_tx_with_file_name(
        curve=SECP256K1,
        path="m/44'/1022'/10'/525'/1238'",
        firmware=firmware, 
        backend=backend,
        navigator=navigator,
        click_count=click_count,
        file_name=file_name,
        test_name=test_name,
        blind_signing_settings=blind_signing_settings
    )


def test_sign_tx_secp256k1_call_function(firmware, backend, navigator, test_name):
    if isinstance(backend, SpeculosBackend):
        enable_blind_signing(navigator) 
    
    sign_tx_secp256k1(
        firmware, backend, navigator, 0, "call_function.txn", test_name,
        blind_signing_settings=BlindSigningSettings.SKIP_IF_OFF
    )

def test_sign_tx_secp256k1_simple_transfer(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 13, "simple_transfer.txn", test_name)

def test_sign_tx_secp256k1_simple_transfer_new_format(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 10, "simple_transfer_new_format.txn", test_name)

def test_sign_tx_secp256k1_simple_transfer_nft(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 13, "simple_transfer_nft.txn", test_name)

def test_sign_tx_secp256k1_simple_transfer_nft_by_id(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 13, "simple_transfer_nft_by_id.txn", test_name)

def test_sign_tx_secp256k1_simple_transfer_nft_new_format(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 13, "simple_transfer_nft_new_format.txn", test_name)

def test_sign_tx_secp256k1_simple_transfer_nft_by_id_new_format(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 13, "simple_transfer_nft_by_id_new_format.txn", test_name)

def test_sign_tx_secp256k1_simple_transfer_with_multiple_locked_fees(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 10, "simple_transfer_with_multiple_locked_fees.txn", test_name)

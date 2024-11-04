from ragger_tests.application_client.curve import SECP256K1
from ragger_tests.test_sign_preauth_raw_ed25519 import sign_preauth_raw_all


def test_sign_preauth_raw_secp256k1_all(firmware, backend, navigator):
    sign_preauth_raw_all(
        curve=SECP256K1,
        path="m/44'/1022'/10'/525'/1238'",
        firmware=firmware, 
        backend=backend,
        navigator=navigator,
    )
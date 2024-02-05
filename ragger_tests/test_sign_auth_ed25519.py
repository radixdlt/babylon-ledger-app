from typing import Generator
from pathlib import Path
from ragger.bip import pack_derivation_path
from ragger.navigator import NavInsID
from contextlib import contextmanager
from cryptography.hazmat.primitives.asymmetric import ed25519

ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

CLA1 = 0xAA
CLA2 = 0xAC
INS = 0x61


def send_derivation_path(backend, path, navigator):
    with backend.exchange_async(cla=CLA1, ins=INS, data=pack_derivation_path(path)) as response:
        navigator.navigate([NavInsID.RIGHT_CLICK])


@contextmanager
def send_auth_request(backend, daddr, origin, nonce) -> Generator[None, None, None]:
    addr_length = len(daddr).to_bytes(1, 'little').hex()
    data = nonce + addr_length + daddr.encode('utf-8').hex() + origin.encode('utf-8').hex()

    with backend.exchange_async(cla=CLA2, ins=INS, data=bytes.fromhex(data)) as response:
        yield response


def sign_auth_ed25519(firmware, backend, navigator, test_name, vector):
    send_derivation_path(backend, "m/44'/1022'/12'/525'/1460'/0'", navigator)

    with send_auth_request(backend, vector[1], vector[2], vector[3]):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                      [NavInsID.RIGHT_CLICK, NavInsID.BOTH_CLICK],
                                                      "Sign Proof?",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name,
                                                      5,
                                                      False,
                                                      True)

    rc = backend.last_async_response.data
    pubkey = ed25519.Ed25519PublicKey.from_public_bytes(bytes(rc[64:96]))
    try:
        pubkey.verify(bytes(rc[0:64]), bytes(rc[96:128]))
    except Exception as e:
        print("Invalid signature ", e)


test_vectors = [
    (
        "dc47fc69e9e45855addf579f398da0309c878092dd95352b9fe187a7e5a529e2",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://dashboard.rdx.works",
        "ec5dcb3d1f75627be1021cb8890f0e8ce0c9fe7f2ff55cbdff096b38a32612c9",
    ),
    (
        "866836f5b9c827ca38fd2bfef94f95ba21933f75a0291c85d3ecfc18b8aa5b2d",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://dashboard.rdx.works",
        "d7fb740b9ff00657d710dcbeddb2d432e697fc0dd39c60feb7858b17ef0eff58",
    ),
    (
        "0f41aa92e8c978d7f920ca56daf123a0a0d975eea06ecfb57bec0a0560fb73e3",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://dashboard.rdx.works",
        "4aaa2ec25c3fe215412b3f005e4c37d518af3a22b4728587cf6dbcf83341e8b3",
    ),
    (
        "9c8d2622cedb9dc4e53daea398dd178a2ec938d402eeaba41a2ac946b0f4dd57",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://stella.swap",
        "a10fad201666b4bcf7f707841d58b11740c290e03790b17ed0fec23b3f180e65",
    ),
    (
        "2c07a4fc72341ae9160a8f9ddf2d0bb8fd9d795ed0d87059a9e5de8321513871",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://stella.swap",
        "718b0eb060a719492011910258a4b4119d8c95aef34eb9519c9fa7de25f7ac43",
    ),
    (
        "306b2407e8b675bb22b630efa938249595433975276862e9bfa07f7f94ca84a8",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://stella.swap",
        "9a4f834aefdc455cb4601337227e1b7e74d60308327564ececf33456509964cd",
    ),
    (
        "a14942b1dc361c7e153e4d4200f902da1dafa2bd54bc4c0387c779c22a1e454e",
        "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
        "https://rola.xrd",
        "00dca15875839ab1f549445a36c7b5c0dcf7aebfa7d48f945f2aa5cf4aa1a9a3",
    ),
    (
        "6a13329619caafdf4351d1c8b85b7f523ce2955873f003402be6e1e45cdce4ae",
        "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
        "https://rola.xrd",
        "0a510b2362c9ce19d11c538b2f6a15f62caab6528071eaad5ba8a563a02e01cb",
    ),
    (
        "f9ec8f328d9aeec55546d1cd78a13cc7967bd52aba3c8e305ed39f82465f395c",
        "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
        "https://rola.xrd",
        "20619c1df905a28e7a76d431f2b59e99dd1a8f386842e1701862e765806a5c47",
    ),
]


def test_sign_auth_ed25519_0(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[0])


def test_sign_auth_ed25519_1(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[1])


def test_sign_auth_ed25519_2(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[2])


def test_sign_auth_ed25519_3(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[3])


def test_sign_auth_ed25519_4(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[4])


def test_sign_auth_ed25519_5(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[5])


def test_sign_auth_ed25519_6(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[6])


def test_sign_auth_ed25519_7(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[7])


def test_sign_auth_ed25519_8(firmware, backend, navigator, test_name):
    sign_auth_ed25519(firmware, backend, navigator, test_name, test_vectors[8])

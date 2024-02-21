from pathlib import Path
from ragger.bip import pack_derivation_path
from ragger.navigator import NavInsID
from cryptography.hazmat.primitives.asymmetric import ed25519


CLA1 = 0xAA
CLA2 = 0xAC
INS = 0x41

DATA_PATH = str(Path(__file__).parent.joinpath("data").absolute()) + "/"


def read_file(file):
    with open(DATA_PATH + file, "rb") as f:
        return f.read()


def send_derivation_path(backend, path, navigator):
    with backend.exchange_async(cla=CLA1, ins=INS, data=pack_derivation_path(path)) as response:
        navigator.navigate([NavInsID.RIGHT_CLICK])


def send_tx_intent(txn, click_count, backend, navigator, firmware):
    num_chunks = len(txn) // 255 + 1
    clicks = [NavInsID.RIGHT_CLICK] * click_count
    clicks.append(NavInsID.BOTH_CLICK)

    for i in range(num_chunks):
        chunk = txn[i * 255:(i + 1) * 255]

        if i != num_chunks - 1:
            cls = 0xAB
            backend.exchange(cla=cls, ins=INS, p1=0, p2=0, data=chunk)
        else:
            cls = 0xAC
            with backend.exchange_async(cla=cls, ins=INS, p1=0, p2=0, data=chunk) as response:
                if firmware.device.startswith("nano"):
                    navigator.navigate(clicks)
    return backend.last_async_response.data


def sign_tx_ed25519(firmware, backend, navigator, click_count, file_name):
    send_derivation_path(backend, "m/44'/1022'/12'/525'/1460'/0'", navigator)
    txn = read_file(file_name)

    rc = send_tx_intent(txn, click_count, backend, navigator, firmware)
    pubkey = ed25519.Ed25519PublicKey.from_public_bytes(bytes(rc[64:96]))
    try:
        pubkey.verify(bytes(rc[0:64]), bytes(rc[96:128]))
    except Exception as e:
        print("Invalid signature ", e)


def test_sign_tx_ed25519_simple_transfer(firmware, backend, navigator):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer.txn")


def test_sign_tx_ed25519_simple_transfer_new_format(firmware, backend, navigator):
    sign_tx_ed25519(firmware, backend, navigator, 10, "simple_transfer_new_format.txn")


def test_sign_tx_ed25519_simple_transfer_nft(firmware, backend, navigator):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft.txn")


def test_sign_tx_ed25519_simple_transfer_nft_by_id(firmware, backend, navigator):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft_by_id.txn")


def test_sign_tx_ed25519_simple_transfer_nft_new_format(firmware, backend, navigator):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft_new_format.txn")


def test_sign_tx_ed25519_simple_transfer_nft_by_id_new_format(firmware, backend, navigator):
    sign_tx_ed25519(firmware, backend, navigator, 13, "simple_transfer_nft_by_id_new_format.txn")


def test_sign_tx_ed25519_simple_transfer_with_multiple_locked_fees(firmware, backend, navigator):
    sign_tx_ed25519(firmware, backend, navigator, 10, "simple_transfer_with_multiple_locked_fees.txn")


from pathlib import Path
from ragger.bip import pack_derivation_path
from ragger.navigator import NavInsID
from cryptography.hazmat.primitives.asymmetric import ec, utils
from cryptography.hazmat.primitives import hashes


CLA1 = 0xAA
CLA2 = 0xAC
INS = 0x51

DATA_PATH = str(Path(__file__).parent.joinpath("data").absolute()) + "/"
ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()


def read_file(file):
    with open(DATA_PATH + file, "rb") as f:
        return f.read()


def send_derivation_path(backend, path, navigator):
    with backend.exchange_async(cla=CLA1, ins=INS, data=pack_derivation_path(path)) as response:
        navigator.navigate([NavInsID.RIGHT_CLICK])


def send_tx_intent(txn, click_count, backend, navigator, firmware, test_name):
    num_chunks = len(txn) // 255 + 1

    if click_count > 0:
        clicks = [NavInsID.RIGHT_CLICK] * click_count
        clicks.append(NavInsID.BOTH_CLICK)
    else:
        clicks = [NavInsID.RIGHT_CLICK]

    for i in range(num_chunks):
        chunk = txn[i * 255:(i + 1) * 255]

        if i != num_chunks - 1:
            cls = 0xAB
            backend.exchange(cla=cls, ins=INS, p1=0, p2=0, data=chunk)
        else:
            cls = 0xAC
            with backend.exchange_async(cla=cls, ins=INS, p1=0, p2=0, data=chunk) as response:
                if firmware.device.startswith("nano"):
                    navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name, clicks)
    return backend.last_async_response.data


def sign_tx_secp256k1(firmware, backend, navigator, click_count, file_name, test_name):
    send_derivation_path(backend, "m/44'/1022'/10'/525'/1238'", navigator)
    txn = read_file(file_name)
    global rc
    try:
        rc = send_tx_intent(txn, click_count, backend, navigator, firmware, test_name)
    except Exception as e:
        if click_count == 0:
            return
        print("Communication error ", e)
        raise

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
        raise 


def test_sign_tx_secp256k1_call_function(firmware, backend, navigator, test_name):
    sign_tx_secp256k1(firmware, backend, navigator, 0, "call_function.txn", test_name)


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

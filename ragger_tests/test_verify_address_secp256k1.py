from typing import Tuple
from ragger.backend.interface import BackendInterface
from ragger.firmware.structs import Firmware
from ragger.navigator.navigator import Navigator

from ragger_tests.application_client.curve import SECP256K1
from ragger_tests.test_verify_address_ed25519 import verify_address

def verify_address_secp256k1(
    firmware: Firmware, 
    backend: BackendInterface, 
    navigator: Navigator,
    test_name: str, 
    vector: Tuple[str, str]
):
    verify_address(
        curve=SECP256K1,
        firmware=firmware,
        backend=backend,
        navigator=navigator,
        test_name=test_name,
        vector=vector
    )


test_vectors = [
    ("m/44'/1022'/0'/0/0'", "account_rdx16x5wz8wmkumuhn49klq0zwgjn9d8xs7n95maxam04vawld2d27jjz0"),
    ("m/44'/1022'/0'/0/1'", "account_rdx16y6q3q6ey64j5qvkex3q0yshtln6z2lmyk254xrjcq393rc0hc79wl"),
    ("m/44'/1022'/0'/0/2'", "account_rdx16y69pp98nqvh5xt36mcu78m4pvptpu65wa3nsp9qphn3lc4kp8r66k"),
    ("m/44'/1022'/0'/0/3'", "account_rdx16x9rxepradk8g4zamprt7gn5vhs5j3v5e49yqt0k2w64a6phgf58qa"),
    ("m/44'/1022'/0'/0/4'", "account_rdx16ywu4yg6ka4hd0nmju8cjsls75a98d238na6x0phxll855tjgt4gyk"),
    ("m/44'/1022'/0'/0/5'", "account_rdx1685mjfygmz8e9k3x2ee7pt67kqr28lhayug0yuzv4g2v32uakaf9t0"),
    ("m/44'/1022'/0'/0/6'", "account_rdx16yz4qrctz6843j79rvffunvdaz6l0kem266ddg45cnvw9x4g7hhuv5"),
    ("m/44'/1022'/0'/0/7'", "account_rdx16x743lcavljkrfqs4y2slhwtnj3dyqn0glf4gq96m26ff64kl4rnc2"),
    ("m/44'/1022'/0'/0/8'", "account_rdx169lnw3a4dtxy2jggv80dt2vn8tcyypyqjsver6322vx9januj7dwhu"),
    ("m/44'/1022'/0'/0/9'", "account_rdx16xss3896jatrp7zcxgtuvsjy3cgs22s3wnv0qqcdp79jdkv6sufl79"),
    ("m/44'/1022'/0'/0/0", "account_rdx168cucf0r7h07hhzezrz8uem7lh0xxhgvrlv8htvqwjdxfn2k4jrhtk"),
    ("m/44'/1022'/0'/0/1", "account_rdx168vfmhy37elgswtcqwsnjn7hh906s04fwwmdqpmal8l7cc6gvxw4mj"),
    ("m/44'/1022'/0'/0/2", "account_rdx16x3fhgxm2kl99tyzf3zg4qj7z9csfayzgenqj52aykvltsmyvkcknl"),
    ("m/44'/1022'/0'/0/3", "account_rdx16xp6eppfc7fw0vmnj552hwjdxtwz9ymeynwm5fh5jx6kx76zmhdvan"),
    ("m/44'/1022'/0'/0/4", "account_rdx16xjyhlxdjt7smrwawhnvxaw2arndkan6kvywq6us60aqet4p8tpm58"),
    ("m/44'/1022'/0'/0/5", "account_rdx169gwv4uq7ftrenpf6z2lxxwq2mcqsplw3ghcs4n0g2a868l88yq4a4"),
    ("m/44'/1022'/0'/0/6", "account_rdx1686u3ytx09nuw3c9nyauvx7jzdlcy2xqsz740axumt2k6v4jh00yc9"),
    ("m/44'/1022'/0'/0/7", "account_rdx16yvcymlwp2ym4q9dj2ltzzq0df5vltx0xtcyanna78j0mejwf7gnwr"),
    ("m/44'/1022'/0'/0/8", "account_rdx16ys0zmzfjfrsjlsjh8rpl8x0zj8jwt0l86up7rnyfwmy0rkd3xc8aw"),
    ("m/44'/1022'/0'/0/9", "account_rdx168l3drrzhjnlc9a57hmvt6rykhj5l3hrljpj7juu52rgdcyuey7xft"),
]


def test_verify_address_secp256k1_0(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[0])

def test_verify_address_secp256k1_1(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[1])

def test_verify_address_secp256k1_2(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[2])

def test_verify_address_secp256k1_3(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[3])

def test_verify_address_secp256k1_4(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[4])

def test_verify_address_secp256k1_5(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[5])

def test_verify_address_secp256k1_6(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[6])

def test_verify_address_secp256k1_7(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[7])

def test_verify_address_secp256k1_8(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[8])

def test_verify_address_secp256k1_9(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[9])

def test_verify_address_secp256k1_10(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[10])

def test_verify_address_secp256k1_11(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[11])

def test_verify_address_secp256k1_12(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[12])

def test_verify_address_secp256k1_13(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[13])

def test_verify_address_secp256k1_14(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[14])

def test_verify_address_secp256k1_15(firmware, backend, navigator, test_name):
    verify_address_secp256k1(firmware, backend, navigator, test_name, test_vectors[15])

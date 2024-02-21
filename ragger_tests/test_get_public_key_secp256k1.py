from ragger.bip import pack_derivation_path

CLA = 0xAA
INS = 0x31


test_vectors = [
    ("m/44'/1022'/0'/0/0'", "03f43fba6541031ef2195f5ba96677354d28147e45b40cde4662bec9162c361f55"),
    ("m/44'/1022'/0'/0/1'", "0206ea8842365421f48ab84e6b1b197010e5a43a527952b11bc6efe772965e97cc"),
    ("m/44'/1022'/0'/0/2'", "024f44df0493977fcc5704c00c5c89932d77a9a0b016680e6a931684e160fb8d99"),
    ("m/44'/1022'/0'/0/3'", "0388485f6889d7ebcf1cf6f6dafc8ae5d224f9e847fac868c2e006e71ff3383a91"),
    ("m/44'/1022'/0'/0/4'", "024128185f801aee4ebe9a70d6290f60051162526551240da1374363b58e2e1e2c"),
    ("m/44'/1022'/0'/0/5'", "03f3f51a028cbed1a2c0c3b1f21abc5354f58e8d5279e817195750b8ddec9334f4"),
    ("m/44'/1022'/0'/0/6'", "0383d0721aac0569c37edafe5edd6e2d320822dc23f9207b263b2319837ed1a89d"),
    ("m/44'/1022'/0'/0/7'", "03af13461247c39e54fab62597701ab06c67edac7f8de4df1283a2645706c0b153"),
    ("m/44'/1022'/0'/0/8'", "0226912f5226f4a7c9c80780f32c6ad406c8b471c4929033e5e1756ca248c5a278"),
    ("m/44'/1022'/0'/0/9'", "035a9825274e30ce325cc3934b4e23b008097bd97f1b0a0ef57f7dc9a33e5760ed"),
    ("m/44'/1022'/0'/0/0", "03bc2ec8f3668c869577bf66b7b48f8dee57b833916aa70966fa4a5029b63bb18f"),
    ("m/44'/1022'/0'/0/1", "03c8a6a5710b5abba09341c24382de3222913120dee5084e887529bf821f3973e2"),
    ("m/44'/1022'/0'/0/2", "02d6b5b132e16160d6337d83408165c49edac7bb0112b1d1b3e96e3f6908f6d0d6"),
    ("m/44'/1022'/0'/0/3", "03ce5f85ad86922fbc217806a79d9f4d8d6a268f3822ffed9533a9fff73a4374b7"),
    ("m/44'/1022'/0'/0/4", "03e2c66201fc7330992d316d847bdbeb561704b70779ce60a4fcff53ffe5b6cb36"),
    ("m/44'/1022'/0'/0/5", "02df71a292057d1f7cda4fbcd252e43907646610cc191b6f44050683f82a7e63de"),
    ("m/44'/1022'/0'/0/6", "03d054f1c3d7982994d9581c496f84b6cdf584c8eff0401da82d8c19ad88e8a768"),
    ("m/44'/1022'/0'/0/7", "03ccf3b2bd4294d7e7e84268003f1e25c4893a482e28fcf00dfc1ff65679541d50"),
    ("m/44'/1022'/0'/0/8", "0284c067d070bfdb790883cab583f13a0b13f9d52eacdb52a5d8588231ce8c8b89"),
    ("m/44'/1022'/0'/0/9", "02e5703b668deebac710118df687296e90da93c19d0db3cce656b6a677ab3e4747"),
]


def call_and_check(backend, vector):
    path, expected_pub_key = vector
    response = backend.exchange(cla=CLA, ins=INS, data=pack_derivation_path(path)).data
    pk = response.hex()
    assert pk == expected_pub_key, "Invalid public key\nExpected: " + expected_pub_key + "\nReceived: " + pk


def test_get_public_key_secp256k1_0(backend):
    call_and_check(backend, test_vectors[0])


def test_get_public_key_secp256k1_1(backend):
    call_and_check(backend, test_vectors[1])


def test_get_public_key_secp256k1_2(backend):
    call_and_check(backend, test_vectors[2])


def test_get_public_key_secp256k1_3(backend):
    call_and_check(backend, test_vectors[3])


def test_get_public_key_secp256k1_4(backend):
    call_and_check(backend, test_vectors[4])


def test_get_public_key_secp256k1_5(backend):
    call_and_check(backend, test_vectors[5])


def test_get_public_key_secp256k1_6(backend):
    call_and_check(backend, test_vectors[6])


def test_get_public_key_secp256k1_7(backend):
    call_and_check(backend, test_vectors[7])


def test_get_public_key_secp256k1_8(backend):
    call_and_check(backend, test_vectors[8])


def test_get_public_key_secp256k1_9(backend):
    call_and_check(backend, test_vectors[9])


def test_get_public_key_secp256k1_10(backend):
    call_and_check(backend, test_vectors[10])


def test_get_public_key_secp256k1_11(backend):
    call_and_check(backend, test_vectors[11])


def test_get_public_key_secp256k1_12(backend):
    call_and_check(backend, test_vectors[12])


def test_get_public_key_secp256k1_13(backend):
    call_and_check(backend, test_vectors[13])


def test_get_public_key_secp256k1_14(backend):
    call_and_check(backend, test_vectors[14])


def test_get_public_key_secp256k1_15(backend):
    call_and_check(backend, test_vectors[15])


def test_get_public_key_secp256k1_16(backend):
    call_and_check(backend, test_vectors[16])


def test_get_public_key_secp256k1_17(backend):
    call_and_check(backend, test_vectors[17])


def test_get_public_key_secp256k1_18(backend):
    call_and_check(backend, test_vectors[18])


def test_get_public_key_secp256k1_19(backend):
    call_and_check(backend, test_vectors[19])

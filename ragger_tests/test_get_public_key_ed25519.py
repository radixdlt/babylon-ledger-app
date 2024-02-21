from ragger.bip import pack_derivation_path

CLA = 0xAA
INS = 0x21

test_vectors = [
    ("m/44'/1022'/12'/525'/1460'/0'", "451152a1cef7be603205086d4ebac0a0b78fda2ff4684b9dea5ca9ef003d4e7d"),
    ("m/44'/1022'/12'/525'/1460'/1'", "0a4b894208a1f6b1bd7e823b59909f01aae0172b534baa2905b25f1bcbbb4f0a"),
    ("m/44'/1022'/12'/525'/1460'/2'", "235c27aafa83376d475040a7eb53ea889ae93bda005ef7f445f221f73b43313e"),
    ("m/44'/1022'/12'/525'/1460'/3'", "cc294e8c43be93012f827cd54a11a2f836e5934c2361d61b5a737adbd42bf030"),
    ("m/44'/1022'/12'/525'/1678'/0'", "2612a6865d354ed285baf4877d671276e6cd8cd81e3f1101c35d16853c204fa4"),
    ("m/44'/1022'/12'/525'/1678'/1'", "2f92b0b43ee39c6c3006b2a5c7cdbdee0c6b6835d76a0dc8da0aeffc741d5c96"),
    ("m/44'/1022'/12'/525'/1678'/2'", "3f23bcce53cf2ea14d238f8473aaf3c7ed3f4047fa20158389eabb651766f8d5"),
    ("m/44'/1022'/12'/525'/1678'/3'", "5b36d055cdd07129ba0b780cd285661d5dae02831a55b408f84f9b72ba95c0a9"),
    ("m/44'/1022'/12'/525'/1391'/0'", "d998153796a745c2f733079c791f4ae93eb96a812b39c9ee7a26eca32fa14905"),
    ("m/44'/1022'/12'/525'/1391'/1'", "94e163e6739fa0c9db3f44c0675f185fdb0f1dddb6d929cc49a199717c0a2da2"),
    ("m/44'/1022'/12'/525'/1391'/2'", "9bd51ee27f37367ee4c7cf18e5b8e1b40ae808d3da0350de152c9db34de778d3"),
    ("m/44'/1022'/12'/525'/1391'/3'", "d9d6fc68321ce02c30122c6ff5c1a8068142703f9dac362cff29bfb814a2130f"),
    ("m/44'/1022'/12'/618'/1460'/0'", "cc129e0d00d365f2269cee259923e904b8c46ef5b28aefc18df8ed20ef42a3eb"),
    ("m/44'/1022'/12'/618'/1460'/1'", "f67ac35c37921579b59f77fe05a1012ad8240092c8fed0d8fe1f96206eb99c3e"),
    ("m/44'/1022'/12'/618'/1460'/2'", "02c8074258844ae4b81d80261fc411e95070526e0d92803fe4f343e00dc89ed5"),
    ("m/44'/1022'/12'/618'/1460'/3'", "fca4f791866a48cb53a269e420fa6b7f192e98fee5f9e8f9009962ca3d9baeb2"),
    ("m/44'/1022'/12'/618'/1678'/0'", "f6f2056e3edb8905be1717c1f8f5204242047875ba548c19d42962366800c1d4"),
    ("m/44'/1022'/12'/618'/1678'/1'", "960ee0acd88b0e7f1e8cb171139c2e0e7b8d776134a103b36034a6991fcac175"),
    ("m/44'/1022'/12'/618'/1678'/2'", "07ba2aa69eee065495d8820ef9d5a94b982370e233f04472900cfb5efdb4fa3d"),
    ("m/44'/1022'/12'/618'/1678'/3'", "b4763c9a25d95f32e5ddefc7006ffc4a6570818bf24aeff581ac60cd82a751ba"),
    ("m/44'/1022'/12'/618'/1391'/0'", "996626245f999a4c500c394036db43f73abb18f46970066ff124c750dc096360"),
    ("m/44'/1022'/12'/618'/1391'/1'", "afe925e5aabfa04fb10640cad2c1f6739b3dc9fb4ddeba6ff28e90854d45962d"),
    ("m/44'/1022'/12'/618'/1391'/2'", "1226b881b66c58015e760852c0cb202869b73e29fbe0b3d22d50af0fa606b14a"),
    ("m/44'/1022'/12'/618'/1391'/3'", "7fa1d448ef608a6e1a8533d816367b4fa0d60c39844bb82dbce1ea266105a275"),
]


def call_and_check(backend, vector):
    path, expected_pub_key = vector
    response = backend.exchange(cla=CLA, ins=INS, data=pack_derivation_path(path)).data
    pk = response.hex()
    assert pk == expected_pub_key, "Invalid public key\nExpected: " + expected_pub_key + "\nReceived: " + pk


def test_get_public_key_ed25519_0(backend):
    call_and_check(backend, test_vectors[0])


def test_get_public_key_ed25519_1(backend):
    call_and_check(backend, test_vectors[1])


def test_get_public_key_ed25519_2(backend):
    call_and_check(backend, test_vectors[2])


def test_get_public_key_ed25519_3(backend):
    call_and_check(backend, test_vectors[3])


def test_get_public_key_ed25519_4(backend):
    call_and_check(backend, test_vectors[4])


def test_get_public_key_ed25519_5(backend):
    call_and_check(backend, test_vectors[5])


def test_get_public_key_ed25519_6(backend):
    call_and_check(backend, test_vectors[6])


def test_get_public_key_ed25519_7(backend):
    call_and_check(backend, test_vectors[7])


def test_get_public_key_ed25519_8(backend):
    call_and_check(backend, test_vectors[8])


def test_get_public_key_ed25519_9(backend):
    call_and_check(backend, test_vectors[9])


def test_get_public_key_ed25519_10(backend):
    call_and_check(backend, test_vectors[10])


def test_get_public_key_ed25519_11(backend):
    call_and_check(backend, test_vectors[11])


def test_get_public_key_ed25519_12(backend):
    call_and_check(backend, test_vectors[12])


def test_get_public_key_ed25519_13(backend):
    call_and_check(backend, test_vectors[13])


def test_get_public_key_ed25519_14(backend):
    call_and_check(backend, test_vectors[14])


def test_get_public_key_ed25519_15(backend):
    call_and_check(backend, test_vectors[15])


def test_get_public_key_ed25519_16(backend):
    call_and_check(backend, test_vectors[16])


def test_get_public_key_ed25519_17(backend):
    call_and_check(backend, test_vectors[17])


def test_get_public_key_ed25519_18(backend):
    call_and_check(backend, test_vectors[18])


def test_get_public_key_ed25519_19(backend):
    call_and_check(backend, test_vectors[19])


def test_get_public_key_ed25519_20(backend):
    call_and_check(backend, test_vectors[20])


def test_get_public_key_ed25519_21(backend):
    call_and_check(backend, test_vectors[21])


def test_get_public_key_ed25519_22(backend):
    call_and_check(backend, test_vectors[22])


def test_get_public_key_ed25519_23(backend):
    call_and_check(backend, test_vectors[23])

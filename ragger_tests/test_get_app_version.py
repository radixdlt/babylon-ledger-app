CLA = 0xAA
INS = 0x10


def test_get_version(backend):
    assert backend.exchange(cla=CLA, ins=INS).data.hex() == "000718"

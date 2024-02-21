CLA = 0xAA
INS = 0x12


def test_get_version(backend):
    assert backend.exchange(cla=CLA, ins=INS).data.hex() == "41ac202687326a4fc6cb677e9fd92d08b91ce46c669950d58790d4d5e583adc0"

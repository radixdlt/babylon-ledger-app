CLA = 0xAA
INS = 0x11


def test_get_device_model(backend):
    assert backend.exchange(cla=CLA, ins=INS).data.hex() in ["00", "01", "02", "04"]

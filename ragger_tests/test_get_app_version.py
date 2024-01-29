import pytest

CLA = 0xAA
INS = 0x10


@pytest.mark.use_on_backend("ledgercomm")
def test_get_version(backend):
    print(backend.exchange(cla=CLA, ins=INS))
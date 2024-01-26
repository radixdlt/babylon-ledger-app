# define the CLA of your application here
CLA = 0xAA
# define an instruction of your application here
INS = 0x10


def test_get_version(backend):
    print(backend.exchange(cla=CLA, ins=INS))
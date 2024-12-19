from ragger.backend.interface import BackendInterface
from ragger_tests.application_client.app import App
from ragger_tests.application_client.response_unpacker import LedgerModel

def test_get_device_model(backend: BackendInterface):
    model = App(backend).get_device_model()
    assert model in [LedgerModel.NANO_X, LedgerModel.NANO_S, LedgerModel.NANO_S_PLUS, LedgerModel.STAX]

from application_client.app import App
from ragger.backend.interface import BackendInterface

def test_get_device_id(backend: BackendInterface):
    app = App(backend)
    actual = app.get_device_id()
    # The expected blake2b hash of public key at m/44'/365' - of mnemonic
    # `"equip will roof matter pink blind book anxiety banner elbow sun young"`
    expected = "41ac202687326a4fc6cb677e9fd92d08b91ce46c669950d58790d4d5e583adc0"
    assert actual == expected, "Wrong DeviceID, maybe you are running on a Ledger which does not have mnemonic `equip will roof matter pink blind book anxiety banner elbow sun young` setup?"

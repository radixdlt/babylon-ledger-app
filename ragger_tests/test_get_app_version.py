from application_client.app import App
from application_client.response_unpacker import Version
from ragger.backend.interface import BackendInterface

def get_version_from_cargo_toml():
    import tomli
    import os
    from pathlib import Path
    DIR = Path(os.path.dirname(os.path.abspath(__file__)))
    PARENT_DIR = DIR.parent
    CARGO_TOML_DIR = os.path.join(PARENT_DIR, 'Cargo.toml') 
    global version
    with open(CARGO_TOML_DIR, "rb") as f:
        data = tomli.load(f)
        major, minor, patch = tuple(map(int, data['package']['version'].split('.')))
        version = Version(major=major, minor=minor, patch=patch)
    return version

def test_get_version(backend: BackendInterface):
    expected = get_version_from_cargo_toml()
    app = App(backend)
    actual = app.get_version()
    assert actual == expected

# def test_get_app_settings(backend: BackendInterface):
#     app = App(backend)
#     settings = app.get_app_settings()
#     assert settings.is_blind_signing_enabled == True
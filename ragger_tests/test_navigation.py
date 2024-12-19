import time
from pathlib import Path
from ragger.navigator import NavInsID

CLA1 = 0xAA
CLA2 = 0xAC
INS = 0x41

DATA_PATH = str(Path(__file__).parent.joinpath("data").absolute()) + "/"
ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()


def test_dashboard_navigation(firmware, backend, navigator, test_name):
    if firmware.is_nano:
        navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name,
                                       [NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK, NavInsID.RIGHT_CLICK,
                                        NavInsID.RIGHT_CLICK, ], screen_change_before_first_instruction=False)

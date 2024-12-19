from ragger.conftest import configuration
import pytest
from ragger.navigator import NavInsID
from ragger.navigator.navigator import Navigator

###########################
### CONFIGURATION START ###
###########################

# You can configure optional parameters by overriding the value of ragger.configuration.OPTIONAL_CONFIGURATION
# Please refer to ragger/conftest/configuration.py for their descriptions and accepted values

configuration.OPTIONAL.CUSTOM_SEED = "equip will roof matter pink blind book anxiety banner elbow sun young"

#########################
### CONFIGURATION END ###
#########################

# Pull all features from the base ragger conftest using the overridden configuration
pytest_plugins = ("ragger.conftest.base_conftest",)


# Notes :
# 1. Remove this fixture once the pending review screen is removed from the app
# 2. This fixture clears the pending review screen before each test
# 3. The scope should be the same as the one configured by BACKEND_SCOPE in 
# ragger/conftest/configuration.py
@pytest.fixture(scope="class", autouse=True)
def clear_pending_review(firmware, navigator: Navigator):
    # Press a button to clear the pending review
    if firmware.is_nano:
        if navigator._backend.compare_screen_with_text("Pending"):
            print("Clearing pending review")
            instructions = [
                NavInsID.BOTH_CLICK,
            ]
            navigator.navigate(instructions,screen_change_before_first_instruction=False)
        else:
            print("No pending review to clear")
    else:
        print("Not Nano")
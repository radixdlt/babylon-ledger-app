# Babylon Ledger App API

All communication is performed using APDU protocol ([see APDU description](apdu.md)) via USB.

## Overview
| API Name              | Instruction Code | Description                                                                                |
|-----------------------|------------------|--------------------------------------------------------------------------------------------|
| GetApplicationVersion | 0x10             | Get application version as 3 version components: __Major__, __Minor__ and __Patch Level__. |
| GetDeviceModel        | 0x11             | Get device model code. __0__ corresponds to Nano S, __1__ - Nano S Plus, __2__ - Nano X    |
| GetDeviceId           | 0x12             | Get device ID - unique device ID derived from seed phrase stored in device                 |


#!/bin/sh
ledgerctl send get_api_version.apdu | cut -b -10 | xxd -r -p && echo

#!/bin/sh

function outcome() {
    ORNAMENT=$(yes "$1 " | head -n 7 | tr -d '\n')
    NEWLINES=$(yes " " | head -n 5)
    SEP="$NEWLINES$ORNAMENT$NEWLINES"
    printf "$SEP BUILD $2 $SEP"
}

function success() {
    outcome "✅" "SUCCESSFUL"
}

function fail() {
    outcome "❌" "FAILED"
}

cargo ledger build $1 && success || fail

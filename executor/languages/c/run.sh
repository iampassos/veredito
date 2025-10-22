#!/bin/sh
set -e

gcc code.c -o binary 2> error.txt || exit 1

set +e

timeout $TIME_LIMIT ./binary < input.txt > output.txt 2>> error.txt
status=$?

case $status in
    0)   exit 0 ;;
    124) exit 124 ;;
    137) exit 137 ;;
    *)   exit 2 ;;
esac

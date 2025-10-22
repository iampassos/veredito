#!/bin/sh
set +e

timeout $TIME_LIMIT python3 code.py < input.txt > output.txt 2> error.txt
status=$?

case $status in
    0)   exit 0 ;;
    1)   exit 2 ;;
    *)   exit $status ;;
esac


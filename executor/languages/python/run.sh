#!/bin/sh
set -e

timeout $TIME_LIMIT sh -c 'python3 code.py < input.txt > output.txt 2> error.txt; CODE=$?; [ $CODE -eq 0 ] && exit 0 || [ $CODE -eq 1 ] && exit 2 || [ $CODE -eq 137 ] && exit 137 || exit $CODE'

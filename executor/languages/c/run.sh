#!/bin/sh
set -e

gcc code.c -o binary 2> error.txt || exit 1

timeout $TIME_LIMIT sh -c './binary < input.txt > output.txt 2>> error.txt; CODE=$?; [ $CODE -eq 0 ] && exit 0 || [ $CODE -eq 137 ] && exit 137 || exit 2'

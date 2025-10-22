#!/bin/sh

gcc code.c -o binary 2> error.txt || exit 1

if [ $? -ne 0 ]; then
    exit 1
fi

start=$(date +%s%3N)

timeout "$TIME_LIMIT" ./binary < input.txt > output.txt 2>> error.txt
status=$?

end=$(date +%s%3N)

echo $((end - start)) > time.txt

case $status in
    0)   exit 0 ;;
    124) exit 124 ;;
    137) exit 137 ;;
    *)   exit 2 ;;
esac

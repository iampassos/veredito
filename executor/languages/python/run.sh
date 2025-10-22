#!/bin/sh
set +e

start=$(date +%s%3N)

timeout "$TIME_LIMIT" python3 code.py < input.txt > output.txt 2> error.txt
status=$?

end=$(date +%s%3N)

echo $((end - start)) > time.txt

case $status in
    0)   exit 0 ;;
    1)   exit 2 ;;
    124) exit 124 ;;
    137) exit 137 ;;
    *)   exit $status ;;
esac

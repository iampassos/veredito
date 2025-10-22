#!/bin/sh

start=$(date +%s%3N)

timeout "$TIME_LIMIT" sh -c '
    if [ -z "$( ls -A './inputs' )" ]; then
        python3 code.py > ./inputs/0.out 2>> error.txt
        exit $?
    else
        for input in ./inputs/*.in; do
            output="${input%.in}.out"
            python3 code.py < "$input" > "$output" 2> error.txt
            status=$?

           if [ $status -ne 0 ]; then
                exit $status
            fi
        done
    fi
'
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

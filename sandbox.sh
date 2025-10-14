#!/bin/bash

cd /submission

gcc ./source.c
if [ $? -ne 0 ]; then
    exit 1
fi

./a.out < input.txt > output.txt
exit $?

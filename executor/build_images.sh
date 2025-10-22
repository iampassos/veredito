#!/bin/bash

for lang in languages/*/; do
    docker build -t "sandbox-$(basename "$lang")" \
        --build-arg USER_ID=$(id -u) \
        --build-arg GROUP_ID=$(id -g) \
        "$lang"
done

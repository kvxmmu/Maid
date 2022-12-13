#!/usr/bin/env bash

ARM64_CC=aarch64-unknown-linux-gnu-gcc
OBJCOPY=aarch64-unknown-linux-gnu-objcopy

function help() {
    echo "$0: usage: $0 path/to/file.c"
    exit
}

function invoke() {
    echo ">> $@"
    $@
}

if [[ -z "$1" ]]; then
    help
fi

file="$1"
output="${file%.c}.bintest"

invoke $ARM64_CC -ffreestanding -nostdlib -Wl,-Ttext=0x0 $file -o $output
invoke $OBJCOPY -O binary $output


#!/bin/sh

RED='\033[1;31m'
RESET='\033[0m'

targets="x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu aarch64-apple-darwin"

installed_targets=$(rustc --print target-list 2>/dev/null)

if [ -z "$installed_targets" ]; then
    echo "Failed to get rustc target list."
    exit 1
fi

missing=0

installed_targets=$(rustup target list --installed)

for target in $targets; do
    if ! echo "$installed_targets" | grep -q "^$target$"; then
        echo $RED"Error: target $target is not installed."$RESET
        missing=1
    fi
done

if [ $missing -eq 1 ]; then
    echo $RED"One or more required targets are missing."$RESET
    exit 1
fi

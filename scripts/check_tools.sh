#!/bin/sh

tools="mkdir cp cargo cross tar hdiutil rustc docker"

missing=0

for tool in $tools; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "Error: '$tool' is not installed or not in PATH."
        missing=1
    fi
done

if [ $missing -eq 1 ]; then
    echo "One or more required tools are missing."
    exit 1
fi


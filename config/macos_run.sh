#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"
DYLD_LIBRARY_PATH="$DIR" "$DIR/roketz" "$@"

#! /bin/bash

VERUS_NUM_THREADS=8

set -e
set -x

. lib.sh

if [ $# -lt 1 ]; then
    echo "usage: $0 <command>"
    exit 1
fi

RESULTS_DIR=$1
VERUS_ENCODING_TAR=$2

print_header "summarizing"
(cd summarize;
    cargo build --release;
    ./target/release/summarize ../$RESULTS_DIR ../$VERUS_ENCODING_TAR)
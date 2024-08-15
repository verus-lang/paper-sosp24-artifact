#! /bin/bash

DAFNY_THREADS=8

VERIFY_NR=1

set -e
set -x

. lib.sh

if [ $# -lt 1 ]; then
    echo "usage: $0 <command>"
    exit 1
fi

RESULTS_DIR=$1

print_header "verifying"

cd $RESULTS_DIR

echo $DAFNY_THREADS > linear-dafny-num-threads.txt

if [ $VERIFY_NR -eq 1 ]; then
    print_step "verifying nr"

    mkdir -p nr && cd nr
    RESULTS_DIR=$(pwd)

    cd /root/ironsync
    rm -R build/* || true
    python3 /root/eval/time.py \
        /root/eval/results/nr/linear-dafny-verification-parallel.output.txt \
        /root/eval/results/nr/linear-dafny-verification-parallel.time.txt \
        make -j$DAFNY_THREADS build/concurrency/node-replication/Interface.i.verified
    rm -R build/* || true
    python3 /root/eval/time.py \
        /root/eval/results/nr/linear-dafny-verification-singlethread.output.txt \
        /root/eval/results/nr/linear-dafny-verification-singlethread.time.txt \
        make -j1 build/concurrency/node-replication/Interface.i.verified

    cd $RESULTS_DIR
    cd .. # results
fi

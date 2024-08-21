#! /bin/bash

# unzip libicu-dev libgomp1 python3

DAFNY_THREADS=8

VERIFY_IRONSHT=1
COUNT_LINES_IRONSHT=1

set -e
set -x

. lib.sh

if [ $# -lt 1 ]; then
    echo "usage: $0 <command>"
    exit 1
fi

RESULTS_DIR=$1

print_header "setting up repos/"

if [ ! -d "repos" ]; then
    mkdir repos
fi

print_header "cloning or updating repositories"

clone_and_update_repository "ironclad" "main" "https://github.com/microsoft/Ironclad.git"

(cd repos;
    if [ ! -d "dafny-3.4.0" ]; then
        mkdir dafny-3.4.0
        cd dafny-3.4.0
        curl -LO https://github.com/dafny-lang/dafny/releases/download/v3.4.0/dafny-3.4.0-x64-ubuntu-16.04.zip
        unzip dafny-3.4.0-x64-ubuntu-16.04.zip
        rm dafny-3.4.0-x64-ubuntu-16.04.zip
    fi)

print_header "verifying"

cd $RESULTS_DIR

echo $DAFNY_THREADS > dafny-num-threads.txt

DAFNY_EXE=../../repos/dafny-3.4.0/dafny/dafny

if [ $VERIFY_IRONSHT -eq 1 ]; then
    print_step "verifying ironsht"

    mkdir -p ironsht && cd ironsht

    IRONSHT_FILES=$(sed -e 's/^/..\/..\/repos\/ironclad\//' ../../ironsht_files.txt | tr '\n' ' ' | tr -d '\r')
    IRONSHT_NONLINEAR_FILES=$(sed -e 's/^/..\/..\/repos\/ironclad\//' ../../ironsht_files_nonlinear.txt | tr '\n' ' ' | tr -d '\r')

    python3 ../../time.py dafny-verification-parallel.output.txt dafny-verification-parallel.time.txt \
        $DAFNY_EXE /compile:0 /arith:5 /noCheating:1 /trace /vcsCores:$DAFNY_THREADS $IRONSHT_FILES
    python3 ../../time.py dafny-verification-parallel-nonlinear.output.txt dafny-verification-parallel-nonlinear.time.txt \
        $DAFNY_EXE /compile:0 /arith:2 /noCheating:1 /trace /vcsCores:$DAFNY_THREADS $IRONSHT_NONLINEAR_FILES
    python3 ../../time.py dafny-verification-singlethread.output.txt dafny-verification-singlethread.time.txt \
        $DAFNY_EXE /compile:0 /arith:5 /noCheating:1 /trace /vcsCores:1 $IRONSHT_FILES
    python3 ../../time.py dafny-verification-singlethread-nonlinear.output.txt dafny-verification-singlethread-nonlinear.time.txt \
        $DAFNY_EXE /compile:0 /arith:2 /noCheating:1 /trace /vcsCores:1 $IRONSHT_NONLINEAR_FILES

    cd ..
fi

if [ $COUNT_LINES_IRONSHT -eq 1 ]; then
    print_step "counting lines for ironsht"

    mkdir -p ironsht && cd ironsht

    # Following ironfleet we modify sloccount to understand that .dfy files are
    # C# files in disguise:
    ../../dafny-ironfleet/fetch-and-modify-sloccount.sh

    export PATH=$PATH:$(realpath sloccount-2.26/)
    ../../dafny-ironfleet/dafny-line-count.py \
        --dafny_executable $(realpath $DAFNY_EXE) \
        --sloccount_executable sloccount \
        --ironfleet_root ../../repos/ironclad/ironfleet \
        --fileset ../../ironsht_files.txt \
        --fileset ../../ironsht_files_nonlinear.txt \
        --results dafny-linecount.json \
        --cache dafny-linecounts.cache

fi

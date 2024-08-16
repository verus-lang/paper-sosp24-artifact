#! /bin/bash

export VERUS_SINGULAR_PATH="/usr/bin/Singular"

VERUS_NUM_THREADS=8

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

clone_and_update_repository "verus-main" "main" "https://github.com/verus-lang/verus.git"
clone_and_update_repository "verus-main-line-count" "main" "https://github.com/verus-lang/verus.git"
clone_and_update_repository "verified-node-replication" "main" "https://github.com/verus-lang/verified-node-replication.git"
clone_and_update_repository "verified-ironkv" "main" "https://github.com/verus-lang/verified-ironkv.git"
clone_and_update_repository "verified-nrkernel" "main" "https://github.com/utaal/verified-nrkernel.git"
clone_and_update_repository "verified-storage" "generic_trait_serialization" "https://github.com/microsoft/verified-storage.git"
clone_and_update_repository "verified-memory-allocator" "main" "https://github.com/verus-lang/verified-memory-allocator.git"

print_header "getting z3"

print_step "z3 for verus-main"

(cd repos/verus-main/source;
    if [ ! -f "z3" ]; then
        bash ./tools/get-z3.sh
    fi)

print_header "building verus and line-count"

print_step "building verus-main"
(cd repos/verus-main/source; . ../tools/activate; vargo build --release --features singular)

# print_step "building verus-nr"
# (cd repos/verus-nr/source; . ../tools/activate; vargo build --release --features singular)

print_step "building line_count"
(cd repos/verus-main-line-count/source/tools/line_count; cargo build --release)

# =====================================================================================================

print_header "verifying"

cd $RESULTS_DIR

echo $VERUS_NUM_THREADS > verus-num-threads.txt

run_verification() {
    local result_dir=$1
    print_step "verifying $result_dir"
    local exe_path=$2
    local num_threads=$3
    local crate_path=$4
    local extra_flags=${@:5}
    
    mkdir -p $result_dir && cd $result_dir
    if [ -f "verus-encoding.tar.gz" ]; then
        rm verus-encoding.tar.gz
    fi

    python3 ../../time.py verus-verification-parallel.json verus-verification-parallel.time.txt \
        $exe_path --emit=dep-info --time-expanded --no-report-long-running --output-json --num-threads=$num_threads $extra_flags \
        $crate_path

    python3 ../../time.py verus-verification-singlethread.json verus-verification-singlethread.time.txt \
        $exe_path --time-expanded --no-report-long-running --output-json --num-threads=1 --log smt $extra_flags \
        $crate_path

    tar -cvzf verus-encoding.tar.gz .verus-log/*.smt*
    
    rm -R .verus-log

    pwd
    
    cd ..
}

count_lines() {
    local result_dir=$1
    print_step "counting lines for $result_dir"
    local d_path=$2

    cd $result_dir

    ../../repos/verus-main-line-count/source/target/release/line_count --json $d_path > verus-linecount.json

    rm $d_path

    cd ..
}

VERUS_MAIN_EXE=../../repos/verus-main/source/target-verus/release/verus
# VERUS_NR_EXE=../../repos/verus-nr/source/target-verus/release/verus

run_verification page-table $VERUS_MAIN_EXE $VERUS_NUM_THREADS ../../repos/verified-nrkernel/page-table/main.rs --cfg feature=\"impl\" --rlimit 60
count_lines page-table main.d

run_verification ironsht $VERUS_MAIN_EXE $VERUS_NUM_THREADS ../../repos/verified-ironkv/ironsht/src/lib.rs --crate-type=lib
count_lines ironsht lib.d

run_verification nr $VERUS_MAIN_EXE $VERUS_NUM_THREADS ../../repos/verified-node-replication/verified-node-replication/src/lib.rs --crate-type=lib
count_lines nr lib.d

print_step "preparing verified-storage"
(cd ../repos/verified-storage;
    cd deps_hack;
    cargo clean;
    cargo +1.76.0 build;
)

run_verification verified-storage $VERUS_MAIN_EXE $VERUS_NUM_THREADS \
  ../../repos/verified-storage/storage_node/src/lib.rs -L dependency=../../repos/verified-storage/deps_hack/target/debug/deps --extern=deps_hack=../../repos/verified-storage/deps_hack/target/debug/libdeps_hack.rlib
count_lines verified-storage lib.d

print_step "preparing mimalloc"
(cd ../repos/verified-memory-allocator;
    cd test_libc;
    cargo clean;
    cargo +1.76.0 build --release;
    cd ..;
    LIBC_RLIB_NAME=$(find ./test_libc/target/release/deps/ -name 'liblibc-*.rlib');
    mkdir -p build;
    cp $LIBC_RLIB_NAME build/liblibc.rlib;
)

run_verification mimalloc $VERUS_MAIN_EXE $VERUS_NUM_THREADS \
    ../../repos/verified-memory-allocator/verus-mimalloc/lib.rs --extern libc=../../repos/verified-memory-allocator/build/liblibc.rlib --rlimit 240
count_lines mimalloc lib.d


cd .. # $RESULTS_DIR

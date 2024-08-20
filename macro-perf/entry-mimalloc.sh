
source "$HOME/.cargo/env"
export PATH=$PATH:/root/eval/verus/source/target-verus/release

apt-get update
apt-get install singular -y

export VERUS_SINGULAR_PATH=/usr/bin/Singular

cd verified-memory-allocator
./build-benchmarks-and-allocators.sh
./compare-benchmarks.sh

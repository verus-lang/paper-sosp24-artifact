
# apt-get update && \
#     apt-get install -y curl git build-essential unzip wget gcc-multilib python3 && \
#     curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" | sh -s -- --default-toolchain none -y
#

apt-get update && \
    apt-get install -y singular \
    libpmem1 libpmemlog1 libpmem-dev libpmemlog-dev llvm-dev clang libclang-dev # verified-storage

source "$HOME/.cargo/env"

bash verus.sh results

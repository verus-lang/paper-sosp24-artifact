
apt-get update && \
    apt-get install -y singular \
    libpmem1 libpmemlog1 libpmem-dev libpmemlog-dev llvm-dev clang libclang-dev # verified-storage

source "$HOME/.cargo/env"

bash verus.sh results

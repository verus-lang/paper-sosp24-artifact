
# apt-get update && \
#     apt-get install -y curl git build-essential unzip wget gcc-multilib python3 libicu-dev libgomp1 && \
#     curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" | sh -s -- --default-toolchain none -y

source "$HOME/.cargo/env"

bash dafny.sh results

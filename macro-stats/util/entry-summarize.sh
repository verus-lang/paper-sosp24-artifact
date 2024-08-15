
# apt-get update && \
#     apt-get install -y curl git build-essential wget python3 && \
#     curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" | sh -s -- --default-toolchain none -y

source "$HOME/.cargo/env"

bash summarize.sh results


# Install packages
apt-get update && apt-get install --no-install-recommends -y \
  build-essential \
  ca-certificates \
  git \
  libgmp-dev \
  libgomp1 \
  make \
  opam \
  python2 \
  vim-tiny \
  curl \
  python3-distutils \
  autoconf \
  sudo \
  default-jre \
  cvc4
  # && rm -rf /var/lib/apt/lists/*

# Initialize opam
(opam init --disable-sandboxing --no-setup --bare && \
  opam switch create 4.14.1)

# Install rustup
(curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" \
  | sh -s -- --default-toolchain none -y)



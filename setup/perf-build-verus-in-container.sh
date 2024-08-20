
source "$HOME/.cargo/env"

cd verus/source

. ../tools/activate
vargo build --release --features singular

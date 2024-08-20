
source "$HOME/.cargo/env"

cd verus/source

. ../bin/activate
vargo build --release --features singular


source "$HOME/.cargo/env"

cd verus/source

. ../tools/activate
./tools/get-z3.sh
vargo build --release --features singular


if [ -d "verifiers" ]; then
  rm -R verifiers
fi

mkdir verifiers; cd verifiers

# Creusot
git clone https://github.com/xldenis/creusot.git
(cd creusot; git checkout 9203a5975184ba6be5a0d0b47ef3adc3029e0dda; \
  . "$HOME/.cargo/env"; \
  cargo install --locked --path cargo-creusot)

(
  eval $(opam env --switch=4.14.1); \
  opam install ocamlgraph.2.1.0 --confirm-level=unsafe-yes && \ 
  opam install alt-ergo.2.4.1 --confirm-level=unsafe-yes && \
  opam install z3.4.8.5-1 --confirm-level=unsafe-yes && \
  opam pin add why3.1.6.0 'git+https://gitlab.inria.fr/why3/why3.git#c51c244ded49abe332635a126f381aedb1c67715' -y && \
  why3 config detect)

# Fstar
(
  eval $(opam env --switch=4.14.1); \
  opam pin add fstar.2024.01.13~dev 'git+https://github.com/FStarLang/FStar.git#1d823c247b578280cd05a7f416f813589334c569' -y && \
  opam pin add karamel.1.0.0 'git+https://github.com/FStarLang/karamel#5c7ac22a85fb0b9ce8c278084665022bf7dbb3f7' -y
)

# Prusti
git clone https://github.com/viperproject/prusti-dev prusti
(cd prusti; git checkout a5c29c994cee03e1ba02c3bc2c2761803571d3f5; \
  ./x.py setup && \
  ./x.py build --release)

# Dafny
curl -LO https://github.com/dafny-lang/dafny/releases/download/v4.3.0/dafny-4.3.0-x64-ubuntu-20.04.zip
unzip dafny-4.3.0-x64-ubuntu-20.04.zip

# Verus
git clone https://github.com/verus-lang/verus.git
(cd verus; git checkout 50d07b5fe4465fed8b76f4d050c945ba5dd17141; \
  . "$HOME/.cargo/env"; \
  cd source; . ../tools/activate; \
  bash ./tools/get-z3.sh && \
  vargo build --release)

cd ..

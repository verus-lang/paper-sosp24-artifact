
. "$HOME/.cargo/env"

eval $(opam env --switch=4.14.1)

. util/verifiers-exe-vars

mkdir -p results

cd linked-list
python3 oneshot.py > ../results/linked-list-oneshot.tex
python3 repeat.py > ../results/linked-list-repeat.csv
python3 errors.py > ../results/linked-list-errors.csv
cd ..

cd doubly-linked-list
python3 oneshot.py > ../results/linked-list-oneshot.tex
python3 repeat.py > ../results/linked-list-repeat.csv
cd ..

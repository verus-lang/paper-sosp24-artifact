
export EVAL_SAMPLES=$1

. "$HOME/.cargo/env"

eval $(opam env --switch=4.14.1)

. util/verifiers-exe-vars

mkdir -p results

sed -i 's/running_provers_max = ./running_provers_max = 1/' /root/.why3.conf

cd linked-list
python3 oneshot.py > ../results/linked-list-oneshot.tex
python3 repeat.py > ../results/linked-list-repeat.csv

sed -i 's/running_provers_max = ./running_provers_max = 8/' /root/.why3.conf
python3 errors.py > ../results/linked-list-errors.csv
cd ..

sed -i 's/running_provers_max = ./running_provers_max = 1/' /root/.why3.conf

cd doubly-linked-list
python3 oneshot.py > ../results/doubly-linked-list-oneshot.tex
python3 repeat.py > ../results/doubly-linked-list-repeat.csv
cd ..


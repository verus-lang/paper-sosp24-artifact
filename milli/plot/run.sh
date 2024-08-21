bash plot/setup_venv.sh

. ./venv3/bin/activate

cd results

python3 ../plot/linked_list_repeat.py
python3 ../plot/doubly_linked_list_repeat.py

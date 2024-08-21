apt-get install python3.10-venv -y

if [ -d "venv3" ]; then
  rm -R venv3
fi

python3 -m venv venv3

. ./venv3/bin/activate

pip install plotnine

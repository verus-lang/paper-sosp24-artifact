set -e

tmpdir_path=$1

cd $tmpdir_path

wc -c .verus-log/* | grep ' total$' | sed 's/ total$//g'

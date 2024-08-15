set -e

tmpdir_path=$1
dir_name=$2

cd $tmpdir_path/$dir_name

for project in ironsht nr verified-storage mimalloc page-table; do
    byte_count=`wc -c $project/verus-encoding/* | grep ' total$' | sed 's/ total$//g'`
    echo $project $byte_count
done

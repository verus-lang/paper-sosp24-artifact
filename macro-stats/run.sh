
print_top_header() {
    local message=$1
    echo -e "\033[35m■■■ $message ■■■\033[0m"
}

mkdir results

print_top_header "collecting results for verus"

docker run --platform=linux/amd64 --rm -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ubuntu-essentials-rust-1.76.0 /bin/bash util/entry-verus.sh

print_top_header "collecting results for dafny"

docker run --platform=linux/amd64 --rm -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ubuntu-essentials-rust-1.76.0 /bin/bash util/entry-dafny.sh

print_top_header "collecting results for linear-dafny"

docker run --platform=linux/amd64 --rm -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ironsync-osdi2023-artifact /bin/bash util/entry-linear-dafny-early.sh
docker run --platform=linux/amd64 --rm -it -v .:/root/eval -v ./repos/ironsync/build:/root/ironsync/build -w /root/eval ghcr.io/utaal/ironsync-osdi2023-artifact /bin/bash util/entry-linear-dafny.sh

print_top_header "summarizing"

docker run --platform=linux/amd64 --rm -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ubuntu-essentials-rust-1.76.0 /bin/bash util/entry-summarize.sh

print_top_header "rendering table"

docker run --platform=linux/amd64 --rm -it -v .:/root/eval -w /root/eval kjarosh/latex:2024.2-small /bin/bash util/entry-render-table.sh


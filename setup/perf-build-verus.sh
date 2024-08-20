
cd /mydata
git clone https://github.com/verus-lang/verus.git

docker run --platform=linux/amd64 --rm -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ubuntu-essentials-rust-1.76.0 /bin/bash setup/perf-build-verus-in-container.sh

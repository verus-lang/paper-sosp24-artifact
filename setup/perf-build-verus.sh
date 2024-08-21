
cd /mydata
git clone https://github.com/verus-lang/verus.git
(cd verus; git checkout 097ac7ed283ae60375cd9b2b6017b3c629883b2b)

docker run --platform=linux/amd64 --rm -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ubuntu-essentials-rust-1.76.0 /bin/bash verus-sosp24-artifact/setup/perf-build-verus-in-container.sh

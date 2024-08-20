
docker rm -f verus-sosp24-milli || true

docker run --name verus-sosp24-milli --platform=linux/amd64 -d -it -v .:/root/eval -w /root/eval ubuntu:22.04 /bin/bash

docker exec verus-sosp24-milli /bin/bash setup/install.sh

docker exec verus-sosp24-milli /bin/bash setup/verifiers.sh

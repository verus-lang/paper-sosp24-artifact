#! /bin/bash

DAFNY_THREADS=8

set -e
set -x

. lib.sh

if [ $# -lt 1 ]; then
    echo "usage: $0 <command>"
    exit 1
fi

RESULTS_DIR=$1

print_header "setting up repos/"

if [ ! -d "repos" ]; then
    mkdir repos
fi

print_header "cloning or updating repositories"

clone_and_update_repository "ironsync" "osdi2023-artifact" "https://github.com/secure-foundations/ironsync-osdi2023.git"


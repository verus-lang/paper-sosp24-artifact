
print_header() {
    local message=$1
    echo -e "\033[32m■ $message\033[0m"
}

print_step() {
    local message=$1
    echo -e "\033[34m  ■ $message\033[0m"
}

clone_and_update_repository() {
    print_step "cloning or updating $1"

    cd repos
    
    local repo_name=$1
    local branch=$2
    local repo_url=$3
    local repo_refspec=$4
    local repo_path="$repo_name"

    if [ ! -d "$repo_path" ]; then
        GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" \
            git clone -b $branch --single-branch --depth 1 $repo_url $repo_path
    fi
    (cd $repo_path;
        GIT_SSH_COMMAND="ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" \
            git fetch origin $branch; \
        git checkout $repo_refspec)

    cd .. # repos
}


rm -rf "$1" && cargo new "$1" && cd $(ls -t | head -1) && cargo build && cd -

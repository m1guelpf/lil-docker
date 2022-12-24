#!/bin/sh

exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/lil_docker \
    --manifest-path "$(dirname "$0")/Cargo.toml" "$@"

#!/bin/bash

pushd $(dirname $0)/../source
echo "Running $1..."
cargo run --verbose
popd

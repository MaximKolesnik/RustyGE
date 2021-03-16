#!/bin/bash

pushd $(dirname $0)/../engine
echo "Running $1..."
cargo run
popd

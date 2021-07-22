#!/bin/bash

pushd $(dirname $0)/../$1
cargo build -Z extra-link-arg
popd

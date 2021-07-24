#!/bin/bash

pushd $(dirname $0)/../source
cargo build -Z extra-link-arg
popd

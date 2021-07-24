#!/bin/bash

file=$(dirname $0)/../$2
echo $file

if [[ -f $file ]]; then
    dir=$(dirname "$file")
else
    dir=$file/
fi

echo $dir

pushd $dir
cargo $1
ec=$?
popd

exit $ec
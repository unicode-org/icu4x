#!/bin/bash

set -e

pushd $(dirname "$0")/../../../../examples/cargo/buffer
make clean bin/tutorial_buffer.wasm
popd
cp $(dirname "$0")/../../../../examples/cargo/buffer/bin/tutorial_buffer.wasm $(dirname "$0")
wasm2wat $(dirname "$0")/tutorial_buffer.wasm -o $(dirname "$0")/tutorial_buffer.wat

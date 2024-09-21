#!/bin/bash

set -e

pushd $(dirname "$0")/../../../../tutorials/rust/buffer
make bin/tutorial_buffer.wasm
popd
cp $(dirname "$0")/../../../../tutorials/rust/buffer/bin/tutorial_buffer.wasm $(dirname "$0")
wasm2wat tutorial_buffer.wasm -o tutorial_buffer.wat

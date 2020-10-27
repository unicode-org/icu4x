cargo +nightly build -Z build-std=std,panic_abort --example unicode_bmp_blocks_selector --release --target wasm32-unknown-unknown &&
mkdir -p pkg &&
cp ../../target/wasm32-unknown-unknown/release/examples/unicode_bmp_blocks_selector.wasm pkg/xmp.wasm &&
wasm-opt pkg/xmp.wasm -Os -o pkg/xmp_opt.wasm &&
twiggy dominators pkg/xmp.wasm > pkg/xmp.txt &&
twiggy dominators pkg/xmp_opt.wasm > pkg/xmp_opt.txt &&
wasm2wat pkg/xmp.wasm -o pkg/xmp.wat &&
wasm2wat pkg/xmp_opt.wasm -o pkg/xmp_opt.wat &&
du -b pkg/*;

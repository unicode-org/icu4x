cargo +nightly build-wasm --example permyriad &&
mkdir -p pkg &&
cp ../../target/wasm32-unknown-unknown/release/examples/permyriad.wasm pkg &&
wasm-opt pkg/permyriad.wasm -Os -o pkg/permyriad_opt.wasm &&
twiggy dominators pkg/permyriad.wasm > pkg/permyriad.txt &&
twiggy dominators pkg/permyriad_opt.wasm > pkg/permyriad_opt.txt &&
wasm2wat pkg/permyriad.wasm -o pkg/permyriad.wat &&
wasm2wat pkg/permyriad_opt.wasm -o pkg/permyriad_opt.wat &&
zip pkg/permyriad.zip pkg/permyriad.wasm &&
zip pkg/permyriad_opt.zip pkg/permyriad_opt.wasm &&
du -b pkg/*;

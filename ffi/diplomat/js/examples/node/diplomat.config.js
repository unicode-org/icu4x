export default {
    wasm_path: new URL('./lib/icu_capi.wasm', import.meta.url),
    init: wasm => wasm.icu4x_init(),
};

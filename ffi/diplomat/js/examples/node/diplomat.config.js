// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

export default {
    wasm_path: new URL('./icu_capi.wasm', import.meta.url),
    init: wasm => wasm.icu4x_init(),
};

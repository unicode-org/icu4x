import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"

export class ICU4XPluralOperands {
  constructor(underlying) {
    this.i = (new BigUint64Array(wasm.memory.buffer, underlying, 1))[0];
    this.v = (new Uint32Array(wasm.memory.buffer, underlying + 8, 1))[0];
    this.w = (new Uint32Array(wasm.memory.buffer, underlying + 12, 1))[0];
    this.f = (new BigUint64Array(wasm.memory.buffer, underlying + 16, 1))[0];
    this.t = (new BigUint64Array(wasm.memory.buffer, underlying + 24, 1))[0];
    this.c = (new Uint32Array(wasm.memory.buffer, underlying + 32, 1))[0];
  }

  static create(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str(wasm, arg_s);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(37, 8);
      wasm.ICU4XPluralOperands_create(diplomat_receive_buffer, buf_arg_s.ptr, buf_arg_s.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 36);
      if (is_ok) {
        const ok_value = new ICU4XPluralOperands(diplomat_receive_buffer);
        wasm.diplomat_free(diplomat_receive_buffer, 37, 8);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 37, 8);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_s.free();
    return diplomat_out;
  }
}

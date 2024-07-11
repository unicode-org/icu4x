import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"

const ComposingNormalizer_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XComposingNormalizer_destroy(underlying);
});

export class ComposingNormalizer {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ComposingNormalizer_box_destroy_registry.register(this, underlying);
    }
  }

  static create_nfc(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XComposingNormalizer_create_nfc(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ComposingNormalizer(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_nfkc(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XComposingNormalizer_create_nfkc(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ComposingNormalizer(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  normalize(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XComposingNormalizer_normalize(this.underlying, buf_arg_s.ptr, buf_arg_s.size, write);
    });
    buf_arg_s.free();
    return diplomat_out;
  }

  is_normalized(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = wasm.ICU4XComposingNormalizer_is_normalized(this.underlying, buf_arg_s.ptr, buf_arg_s.size);
    buf_arg_s.free();
    return diplomat_out;
  }

  is_normalized_utf16(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str16(wasm, arg_s);
    const diplomat_out = wasm.ICU4XComposingNormalizer_is_normalized_utf16(this.underlying, buf_arg_s.ptr, buf_arg_s.size);
    buf_arg_s.free();
    return diplomat_out;
  }

  is_normalized_up_to(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = wasm.ICU4XComposingNormalizer_is_normalized_up_to(this.underlying, buf_arg_s.ptr, buf_arg_s.size);
    buf_arg_s.free();
    return diplomat_out;
  }

  is_normalized_utf16_up_to(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str16(wasm, arg_s);
    const diplomat_out = wasm.ICU4XComposingNormalizer_is_normalized_utf16_up_to(this.underlying, buf_arg_s.ptr, buf_arg_s.size);
    buf_arg_s.free();
    return diplomat_out;
  }
}

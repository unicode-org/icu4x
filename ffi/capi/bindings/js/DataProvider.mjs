import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"

const DataProvider_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDataProvider_destroy(underlying);
});

export class DataProvider {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      DataProvider_box_destroy_registry.register(this, underlying);
    }
  }

  static create_compiled() {
    return new DataProvider(wasm.ICU4XDataProvider_create_compiled(), true, []);
  }

  static create_fs(arg_path) {
    const buf_arg_path = diplomatRuntime.DiplomatBuf.str8(wasm, arg_path);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDataProvider_create_fs(diplomat_receive_buffer, buf_arg_path.ptr, buf_arg_path.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new DataProvider(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_path.free();
    return diplomat_out;
  }

  static create_from_byte_slice(arg_blob) {
    const buf_arg_blob = diplomatRuntime.DiplomatBuf.slice(wasm, arg_blob, "u8");
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDataProvider_create_from_byte_slice(diplomat_receive_buffer, buf_arg_blob.ptr, buf_arg_blob.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new DataProvider(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_blob.leak();
    return diplomat_out;
  }

  static create_empty() {
    return new DataProvider(wasm.ICU4XDataProvider_create_empty(), true, []);
  }

  fork_by_key(arg_other) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDataProvider_fork_by_key(diplomat_receive_buffer, this.underlying, arg_other.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  fork_by_locale(arg_other) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDataProvider_fork_by_locale(diplomat_receive_buffer, this.underlying, arg_other.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  enable_locale_fallback_with(arg_fallbacker) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDataProvider_enable_locale_fallback_with(diplomat_receive_buffer, this.underlying, arg_fallbacker.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = {};
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }
}

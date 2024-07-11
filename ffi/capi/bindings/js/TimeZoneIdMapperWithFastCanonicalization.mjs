import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { TimeZoneInvalidIdError_js_to_rust, TimeZoneInvalidIdError_rust_to_js } from "./TimeZoneInvalidIdError.mjs"

const TimeZoneIdMapperWithFastCanonicalization_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XTimeZoneIdMapperWithFastCanonicalization_destroy(underlying);
});

export class TimeZoneIdMapperWithFastCanonicalization {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      TimeZoneIdMapperWithFastCanonicalization_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XTimeZoneIdMapperWithFastCanonicalization_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new TimeZoneIdMapperWithFastCanonicalization(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  canonicalize_iana(arg_value) {
    const buf_arg_value = diplomatRuntime.DiplomatBuf.str8(wasm, arg_value);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana(diplomat_receive_buffer, this.underlying, buf_arg_value.ptr, buf_arg_value.size, write);
        const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
        if (is_ok) {
          const ok_value = {};
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          return ok_value;
        } else {
          const throw_value = TimeZoneInvalidIdError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    buf_arg_value.free();
    return diplomat_out;
  }

  canonical_iana_from_bcp47(arg_value) {
    const buf_arg_value = diplomatRuntime.DiplomatBuf.str8(wasm, arg_value);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47(diplomat_receive_buffer, this.underlying, buf_arg_value.ptr, buf_arg_value.size, write);
        const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
        if (is_ok) {
          const ok_value = {};
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          return ok_value;
        } else {
          const throw_value = TimeZoneInvalidIdError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    buf_arg_value.free();
    return diplomat_out;
  }
}

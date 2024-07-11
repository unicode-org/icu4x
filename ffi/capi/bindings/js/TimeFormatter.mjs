import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { Error_js_to_rust, Error_rust_to_js } from "./Error.mjs"
import { TimeLength_js_to_rust, TimeLength_rust_to_js } from "./TimeLength.mjs"

const TimeFormatter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XTimeFormatter_destroy(underlying);
});

export class TimeFormatter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      TimeFormatter_box_destroy_registry.register(this, underlying);
    }
  }

  static create_with_length(arg_provider, arg_locale, arg_length) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XTimeFormatter_create_with_length(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, TimeLength_js_to_rust[arg_length]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new TimeFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = Error_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  format_time(arg_value) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XTimeFormatter_format_time(this.underlying, arg_value.underlying, write);
    });
  }

  format_datetime(arg_value) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XTimeFormatter_format_datetime(this.underlying, arg_value.underlying, write);
    });
  }

  format_iso_datetime(arg_value) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XTimeFormatter_format_iso_datetime(this.underlying, arg_value.underlying, write);
    });
  }
}

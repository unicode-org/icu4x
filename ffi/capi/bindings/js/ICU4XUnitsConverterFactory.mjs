import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.mjs"
import { ICU4XMeasureUnitParser } from "./ICU4XMeasureUnitParser.mjs"
import { ICU4XUnitsConverter } from "./ICU4XUnitsConverter.mjs"

const ICU4XUnitsConverterFactory_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XUnitsConverterFactory_destroy(underlying);
});

export class ICU4XUnitsConverterFactory {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XUnitsConverterFactory_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XUnitsConverterFactory_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XUnitsConverterFactory(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  converter(arg_input_unit, arg_output_unit) {
    return (() => {
      const option_ptr = wasm.ICU4XUnitsConverterFactory_converter(this.underlying, arg_input_unit.underlying, arg_output_unit.underlying);
      return (option_ptr == 0) ? undefined : new ICU4XUnitsConverter(option_ptr, true, []);
    })();
  }

  parser() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XUnitsConverterFactory_parser(diplomat_receive_buffer, this.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XMeasureUnitParser(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }
}

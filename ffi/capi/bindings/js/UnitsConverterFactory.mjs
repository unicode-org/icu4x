import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { MeasureUnitParser } from "./MeasureUnitParser.mjs"
import { UnitsConverter } from "./UnitsConverter.mjs"

const UnitsConverterFactory_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XUnitsConverterFactory_destroy(underlying);
});

export class UnitsConverterFactory {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      UnitsConverterFactory_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XUnitsConverterFactory_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new UnitsConverterFactory(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  converter(arg_from, arg_to) {
    return (() => {
      const option_ptr = wasm.ICU4XUnitsConverterFactory_converter(this.underlying, arg_from.underlying, arg_to.underlying);
      return (option_ptr == 0) ? undefined : new UnitsConverter(option_ptr, true, []);
    })();
  }

  parser() {
    return new MeasureUnitParser(wasm.ICU4XUnitsConverterFactory_parser(this.underlying), true, [this]);
  }
}

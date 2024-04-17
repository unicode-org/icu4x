import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.mjs"
import { ICU4XMeasureUnit } from "./ICU4XMeasureUnit.mjs"
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

  parse(arg_unit_id) {
    const buf_arg_unit_id = diplomatRuntime.DiplomatBuf.str8(wasm, arg_unit_id);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XUnitsConverterFactory_parse(diplomat_receive_buffer, this.underlying, buf_arg_unit_id.ptr, buf_arg_unit_id.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XMeasureUnit(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_unit_id.free();
    return diplomat_out;
  }

  convert_f64(arg_input) {
    return wasm.ICU4XUnitsConverterFactory_convert_f64(this.underlying, arg_input);
  }
}

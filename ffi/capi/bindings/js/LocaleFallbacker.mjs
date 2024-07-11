import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { LocaleFallbackPriority_js_to_rust, LocaleFallbackPriority_rust_to_js } from "./LocaleFallbackPriority.mjs"
import { LocaleFallbackSupplement_js_to_rust, LocaleFallbackSupplement_rust_to_js } from "./LocaleFallbackSupplement.mjs"
import { LocaleFallbackerWithConfig } from "./LocaleFallbackerWithConfig.mjs"
import { LocaleParseError_js_to_rust, LocaleParseError_rust_to_js } from "./LocaleParseError.mjs"

const LocaleFallbacker_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocaleFallbacker_destroy(underlying);
});

export class LocaleFallbacker {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      LocaleFallbacker_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocaleFallbacker_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LocaleFallbacker(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_without_data() {
    return new LocaleFallbacker(wasm.ICU4XLocaleFallbacker_create_without_data(), true, []);
  }

  for_config(arg_config) {
    const field_priority_arg_config = arg_config["priority"];
    const field_extension_key_arg_config = arg_config["extension_key"];
    const buf_field_extension_key_arg_config = diplomatRuntime.DiplomatBuf.str8(wasm, field_extension_key_arg_config);
    const field_fallback_supplement_arg_config = arg_config["fallback_supplement"];
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocaleFallbacker_for_config(diplomat_receive_buffer, this.underlying, LocaleFallbackPriority_js_to_rust[field_priority_arg_config], buf_field_extension_key_arg_config.ptr, buf_field_extension_key_arg_config.size, LocaleFallbackSupplement_js_to_rust[field_fallback_supplement_arg_config]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LocaleFallbackerWithConfig(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, [this]);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = LocaleParseError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_field_extension_key_arg_config.free();
    return diplomat_out;
  }
}

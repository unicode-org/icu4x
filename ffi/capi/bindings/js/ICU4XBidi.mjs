import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XBidiInfo } from "./ICU4XBidiInfo.mjs"
import { ICU4XDataError_js_to_rust, ICU4XDataError_rust_to_js } from "./ICU4XDataError.mjs"
import { ICU4XReorderedIndexMap } from "./ICU4XReorderedIndexMap.mjs"

const ICU4XBidi_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XBidi_destroy(underlying);
});

export class ICU4XBidi {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XBidi_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XBidi_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XBidi(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XDataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  for_text(arg_text, arg_default_level) {
    const buf_arg_text = diplomatRuntime.DiplomatBuf.str8(wasm, arg_text);
    const diplomat_out = (() => {
      const option_ptr = wasm.ICU4XBidi_for_text(this.underlying, buf_arg_text.ptr, buf_arg_text.size, arg_default_level);
      return (option_ptr == 0) ? undefined : new ICU4XBidiInfo(option_ptr, true, [buf_arg_text]);
    })();
    buf_arg_text.garbageCollect();
    return diplomat_out;
  }

  reorder_visual(arg_levels) {
    const buf_arg_levels = diplomatRuntime.DiplomatBuf.slice(wasm, arg_levels, "u8");
    const diplomat_out = new ICU4XReorderedIndexMap(wasm.ICU4XBidi_reorder_visual(this.underlying, buf_arg_levels.ptr, buf_arg_levels.size), true, []);
    buf_arg_levels.free();
    return diplomat_out;
  }

  static level_is_rtl(arg_level) {
    return wasm.ICU4XBidi_level_is_rtl(arg_level);
  }

  static level_is_ltr(arg_level) {
    return wasm.ICU4XBidi_level_is_ltr(arg_level);
  }

  static level_rtl() {
    return wasm.ICU4XBidi_level_rtl();
  }

  static level_ltr() {
    return wasm.ICU4XBidi_level_ltr();
  }
}

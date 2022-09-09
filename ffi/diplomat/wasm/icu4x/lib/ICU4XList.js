import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

const ICU4XList_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XList_destroy(underlying);
});

export class ICU4XList {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XList_box_destroy_registry.register(this, underlying);
    }
  }

  static new() {
    return new ICU4XList(wasm.ICU4XList_new(), true, []);
  }

  static with_capacity(arg_capacity) {
    return new ICU4XList(wasm.ICU4XList_with_capacity(arg_capacity), true, []);
  }

  push(arg_val) {
    const buf_arg_val = diplomatRuntime.DiplomatBuf.str(wasm, arg_val);
    wasm.ICU4XList_push(this.underlying, buf_arg_val.ptr, buf_arg_val.size);
    buf_arg_val.free();
  }

  len() {
    return wasm.ICU4XList_len(this.underlying);
  }
}

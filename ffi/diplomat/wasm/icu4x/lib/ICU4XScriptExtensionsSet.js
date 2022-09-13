import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

const ICU4XScriptExtensionsSet_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XScriptExtensionsSet_destroy(underlying);
});

export class ICU4XScriptExtensionsSet {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XScriptExtensionsSet_box_destroy_registry.register(this, underlying);
    }
  }

  contains(arg_script) {
    return wasm.ICU4XScriptExtensionsSet_contains(this.underlying, arg_script);
  }
}

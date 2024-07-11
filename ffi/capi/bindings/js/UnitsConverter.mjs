import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const UnitsConverter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XUnitsConverter_destroy(underlying);
});

export class UnitsConverter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      UnitsConverter_box_destroy_registry.register(this, underlying);
    }
  }

  convert_f64(arg_value) {
    return wasm.ICU4XUnitsConverter_convert_f64(this.underlying, arg_value);
  }

  clone() {
    return new UnitsConverter(wasm.ICU4XUnitsConverter_clone(this.underlying), true, []);
  }
}

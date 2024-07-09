import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XMeasureUnit } from "./ICU4XMeasureUnit.mjs"

const ICU4XMeasureUnitParser_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XMeasureUnitParser_destroy(underlying);
});

export class ICU4XMeasureUnitParser {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XMeasureUnitParser_box_destroy_registry.register(this, underlying);
    }
  }

  parse(arg_unit_id) {
    const buf_arg_unit_id = diplomatRuntime.DiplomatBuf.str8(wasm, arg_unit_id);
    const diplomat_out = (() => {
      const option_ptr = wasm.ICU4XMeasureUnitParser_parse(this.underlying, buf_arg_unit_id.ptr, buf_arg_unit_id.size);
      return (option_ptr == 0) ? undefined : new ICU4XMeasureUnit(option_ptr, true, []);
    })();
    buf_arg_unit_id.free();
    return diplomat_out;
  }
}

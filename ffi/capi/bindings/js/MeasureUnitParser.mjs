import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { MeasureUnit } from "./MeasureUnit.mjs"

const MeasureUnitParser_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XMeasureUnitParser_destroy(underlying);
});

export class MeasureUnitParser {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      MeasureUnitParser_box_destroy_registry.register(this, underlying);
    }
  }

  parse(arg_unit_id) {
    const buf_arg_unit_id = diplomatRuntime.DiplomatBuf.str8(wasm, arg_unit_id);
    const diplomat_out = (() => {
      const option_ptr = wasm.ICU4XMeasureUnitParser_parse(this.underlying, buf_arg_unit_id.ptr, buf_arg_unit_id.size);
      return (option_ptr == 0) ? undefined : new MeasureUnit(option_ptr, true, []);
    })();
    buf_arg_unit_id.free();
    return diplomat_out;
  }
}

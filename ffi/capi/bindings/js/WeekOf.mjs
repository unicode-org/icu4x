import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { WeekRelativeUnit_js_to_rust, WeekRelativeUnit_rust_to_js } from "./WeekRelativeUnit.mjs"

export class WeekOf {
  constructor(underlying) {
    this.week = (new Uint16Array(wasm.memory.buffer, underlying, 1))[0];
    this.unit = WeekRelativeUnit_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
  }
}

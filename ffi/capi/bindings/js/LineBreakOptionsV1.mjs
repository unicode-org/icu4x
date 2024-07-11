import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { LineBreakStrictness_js_to_rust, LineBreakStrictness_rust_to_js } from "./LineBreakStrictness.mjs"
import { LineBreakWordOption_js_to_rust, LineBreakWordOption_rust_to_js } from "./LineBreakWordOption.mjs"

export class LineBreakOptionsV1 {
  constructor(underlying) {
    this.strictness = LineBreakStrictness_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.word_option = LineBreakWordOption_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
    this.ja_zh = (new Uint8Array(wasm.memory.buffer, underlying + 8, 1))[0] == 1;
  }
}

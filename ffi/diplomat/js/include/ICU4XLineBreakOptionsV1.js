import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XLineBreakStrictness_js_to_rust, ICU4XLineBreakStrictness_rust_to_js } from "./ICU4XLineBreakStrictness.js"
import { ICU4XWordBreakRule_js_to_rust, ICU4XWordBreakRule_rust_to_js } from "./ICU4XWordBreakRule.js"

export class ICU4XLineBreakOptionsV1 {
  constructor(underlying) {
    this.line_break_rule = ICU4XLineBreakStrictness_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.word_break_rule = ICU4XWordBreakRule_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
    this.ja_zh = (new Uint8Array(wasm.memory.buffer, underlying + 8, 1))[0] == 1;
  }
}

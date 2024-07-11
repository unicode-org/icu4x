import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { CollatorAlternateHandling_js_to_rust, CollatorAlternateHandling_rust_to_js } from "./CollatorAlternateHandling.mjs"
import { CollatorBackwardSecondLevel_js_to_rust, CollatorBackwardSecondLevel_rust_to_js } from "./CollatorBackwardSecondLevel.mjs"
import { CollatorCaseFirst_js_to_rust, CollatorCaseFirst_rust_to_js } from "./CollatorCaseFirst.mjs"
import { CollatorCaseLevel_js_to_rust, CollatorCaseLevel_rust_to_js } from "./CollatorCaseLevel.mjs"
import { CollatorMaxVariable_js_to_rust, CollatorMaxVariable_rust_to_js } from "./CollatorMaxVariable.mjs"
import { CollatorNumeric_js_to_rust, CollatorNumeric_rust_to_js } from "./CollatorNumeric.mjs"
import { CollatorStrength_js_to_rust, CollatorStrength_rust_to_js } from "./CollatorStrength.mjs"

export class CollatorResolvedOptionsV1 {
  constructor(underlying) {
    this.strength = CollatorStrength_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.alternate_handling = CollatorAlternateHandling_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
    this.case_first = CollatorCaseFirst_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 8)];
    this.max_variable = CollatorMaxVariable_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 12)];
    this.case_level = CollatorCaseLevel_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 16)];
    this.numeric = CollatorNumeric_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 20)];
    this.backward_second_level = CollatorBackwardSecondLevel_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 24)];
  }
}

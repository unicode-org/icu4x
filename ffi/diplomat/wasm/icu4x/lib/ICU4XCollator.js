import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XCollatorAlternateHandling_js_to_rust, ICU4XCollatorAlternateHandling_rust_to_js } from "./ICU4XCollatorAlternateHandling.js"
import { ICU4XCollatorBackwardSecondLevel_js_to_rust, ICU4XCollatorBackwardSecondLevel_rust_to_js } from "./ICU4XCollatorBackwardSecondLevel.js"
import { ICU4XCollatorCaseFirst_js_to_rust, ICU4XCollatorCaseFirst_rust_to_js } from "./ICU4XCollatorCaseFirst.js"
import { ICU4XCollatorCaseLevel_js_to_rust, ICU4XCollatorCaseLevel_rust_to_js } from "./ICU4XCollatorCaseLevel.js"
import { ICU4XCollatorMaxVariable_js_to_rust, ICU4XCollatorMaxVariable_rust_to_js } from "./ICU4XCollatorMaxVariable.js"
import { ICU4XCollatorNumeric_js_to_rust, ICU4XCollatorNumeric_rust_to_js } from "./ICU4XCollatorNumeric.js"
import { ICU4XCollatorStrength_js_to_rust, ICU4XCollatorStrength_rust_to_js } from "./ICU4XCollatorStrength.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"

const ICU4XCollator_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCollator_destroy(underlying);
});

export class ICU4XCollator {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XCollator_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new(arg_provider, arg_locale, arg_options) {
    const field_strength_arg_options = arg_options["strength"];
    const field_alternate_handling_arg_options = arg_options["alternate_handling"];
    const field_case_first_arg_options = arg_options["case_first"];
    const field_max_variable_arg_options = arg_options["max_variable"];
    const field_case_level_arg_options = arg_options["case_level"];
    const field_numeric_arg_options = arg_options["numeric"];
    const field_backward_second_level_arg_options = arg_options["backward_second_level"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCollator_try_new(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, ICU4XCollatorStrength_js_to_rust[field_strength_arg_options], ICU4XCollatorAlternateHandling_js_to_rust[field_alternate_handling_arg_options], ICU4XCollatorCaseFirst_js_to_rust[field_case_first_arg_options], ICU4XCollatorMaxVariable_js_to_rust[field_max_variable_arg_options], ICU4XCollatorCaseLevel_js_to_rust[field_case_level_arg_options], ICU4XCollatorNumeric_js_to_rust[field_numeric_arg_options], ICU4XCollatorBackwardSecondLevel_js_to_rust[field_backward_second_level_arg_options]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XCollator(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }
}

import { CollatorAlternateHandling } from "./CollatorAlternateHandling";
import { CollatorBackwardSecondLevel } from "./CollatorBackwardSecondLevel";
import { CollatorCaseFirst } from "./CollatorCaseFirst";
import { CollatorCaseLevel } from "./CollatorCaseLevel";
import { CollatorMaxVariable } from "./CollatorMaxVariable";
import { CollatorNumeric } from "./CollatorNumeric";
import { CollatorStrength } from "./CollatorStrength";

/**

 * See the {@link https://docs.rs/icu/latest/icu/collator/struct.CollatorOptions.html Rust documentation for `CollatorOptions`} for more information.
 */
export class CollatorOptionsV1 {
  strength: CollatorStrength;
  alternate_handling: CollatorAlternateHandling;
  case_first: CollatorCaseFirst;
  max_variable: CollatorMaxVariable;
  case_level: CollatorCaseLevel;
  numeric: CollatorNumeric;
  backward_second_level: CollatorBackwardSecondLevel;
}

import { FFIError } from "./diplomat-runtime"
import { ICU4XCollatorOptions } from "./ICU4XCollatorOptions";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_collator/struct.Collator.html Rust documentation for `Collator`} for more information.
 */
export class ICU4XCollator {

  /**
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider, locale: ICU4XLocale, options: ICU4XCollatorOptions): ICU4XCollator | never;
}

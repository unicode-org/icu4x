import { char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XCodePointSetBuilder } from "./ICU4XCodePointSetBuilder";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html Rust documentation for `CaseMapCloser`} for more information.
 */
export class ICU4XCaseMapCloser {

  /**

   * Construct a new ICU4XCaseMapper instance

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create(provider: ICU4XDataProvider): ICU4XCaseMapCloser | never;

  /**

   * Adds all simple case mappings and the full case folding for `c` to `builder`. Also adds special case closure mappings.

   * In other words, this adds all characters that this casemaps to, as well as all characters that may casemap to this one.

   * Note that since ICU4XCodePointSetBuilder does not contain strings, this will ignore string mappings

   * Identical to the similarly named method on `ICU4XCaseMapCloser`, use that if you do not plan on using string case closure mappings to limit the amount of data loaded.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_case_closure Rust documentation for `add_case_closure`} for more information.
   */
  add_case_closure(c: char, builder: ICU4XCodePointSetBuilder): void;

  /**

   * Maps the string to single code points and adds the associated case closure mappings, if they exist.

   * The string is mapped to code points if it is their full case folding string. In other words, this performs a reverse full case folding and then adds the case closure items of the resulting code points. If the string is found and its closure applied, then the string itself is added as well as part of its code points' closure.

   * Returns true if the string was found

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_string_case_closure Rust documentation for `add_string_case_closure`} for more information.
   */
  add_string_case_closure(s: string, builder: ICU4XCodePointSetBuilder): boolean;
}

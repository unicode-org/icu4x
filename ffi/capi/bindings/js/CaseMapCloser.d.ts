import { char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { CodePointSetBuilder } from "./CodePointSetBuilder";
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";

/**

 * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html Rust documentation for `CaseMapCloser`} for more information.
 */
export class CaseMapCloser {

  /**

   * Construct a new CaseMapper instance

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider): CaseMapCloser | never;

  /**

   * Adds all simple case mappings and the full case folding for `c` to `builder`. Also adds special case closure mappings.

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_case_closure_to Rust documentation for `add_case_closure_to`} for more information.
   */
  add_case_closure_to(c: char, builder: CodePointSetBuilder): void;

  /**

   * Finds all characters and strings which may casemap to `s` as their full case folding string and adds them to the set.

   * Returns true if the string was found

   * See the {@link https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_string_case_closure_to Rust documentation for `add_string_case_closure_to`} for more information.
   */
  add_string_case_closure_to(s: string, builder: CodePointSetBuilder): boolean;
}

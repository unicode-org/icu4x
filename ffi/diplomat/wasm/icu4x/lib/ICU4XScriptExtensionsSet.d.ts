import { u16 } from "./diplomat-runtime"

/**

 * An object that represents the Script_Extensions property for a single character

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html Rust documentation for `ScriptExtensionsSet`} for more information.
 */
export class ICU4XScriptExtensionsSet {

  /**

   * Check if the Script_Extensions property of the given code point covers the given script

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html#method.contains Rust documentation for `contains`} for more information.
   */
  contains(script: u16): boolean;
}

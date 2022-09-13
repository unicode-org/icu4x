
/**

 * Priority mode for the ICU4X fallback algorithm.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider/enum.FallbackPriority.html Rust documentation for `FallbackPriority`} for more information.
 */
export enum ICU4XLocaleFallbackPriority {
  /**
   */
  Language = 'Language',
  /**
   */
  Region = 'Region',
  /**
   */
  Collation = 'Collation',
}
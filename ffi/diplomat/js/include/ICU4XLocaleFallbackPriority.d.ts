
/**

 * Priority mode for the ICU4X fallback algorithm.

 * See the {@link https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/enum.FallbackPriority.html Rust documentation for `FallbackPriority`} for more information.
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
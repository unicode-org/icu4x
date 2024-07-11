
/**

 * What additional data is required to load when performing fallback.

 * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/enum.LocaleFallbackSupplement.html Rust documentation for `LocaleFallbackSupplement`} for more information.
 */
export enum LocaleFallbackSupplement {
  /**
   */
  None = 'None',
  /**
   */
  Collation = 'Collation',
}
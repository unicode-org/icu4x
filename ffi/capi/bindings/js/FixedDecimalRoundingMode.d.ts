
/**

 * Mode used in a rounding operation.

 * See the {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.RoundingMode.html Rust documentation for `RoundingMode`} for more information.
 */
export enum FixedDecimalRoundingMode {
  /**
   */
  Ceil = 'Ceil',
  /**
   */
  Expand = 'Expand',
  /**
   */
  Floor = 'Floor',
  /**
   */
  Trunc = 'Trunc',
  /**
   */
  HalfCeil = 'HalfCeil',
  /**
   */
  HalfExpand = 'HalfExpand',
  /**
   */
  HalfFloor = 'HalfFloor',
  /**
   */
  HalfTrunc = 'HalfTrunc',
  /**
   */
  HalfEven = 'HalfEven',
}
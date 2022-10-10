
/**

 * ECMA-402 compatible sign display preference.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/enum.SignDisplay.html Rust documentation for `SignDisplay`} for more information.
 */
export enum ICU4XFixedDecimalSignDisplay {
  /**
   */
  Auto = 'Auto',
  /**
   */
  Never = 'Never',
  /**
   */
  Always = 'Always',
  /**
   */
  ExceptZero = 'ExceptZero',
  /**
   */
  Negative = 'Negative',
}
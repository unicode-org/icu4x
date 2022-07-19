
/**

 * The sign of a FixedDecimal, as shown in formatting.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/enum.Sign.html Rust documentation} for more information.
 */
export enum ICU4XFixedDecimalSign {
  /**

   * No sign (implicitly positive, e.g., 1729).
   */
  None = 'None',
  /**

   * A negative sign, e.g., -1729.
   */
  Negative = 'Negative',
  /**

   * An explicit positive sign, e.g., +1729.
   */
  Positive = 'Positive',
}
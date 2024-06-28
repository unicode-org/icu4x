
/**

 * Additional information: {@link https://docs.rs/icu/latest/icu/provider/struct.DataError.html 1}, {@link https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html 2}
 */
export enum ICU4XDataError {
  /**
   */
  Unknown = 'Unknown',
  /**
   */
  MarkerNotFound = 'MarkerNotFound',
  /**
   */
  IdentifierNotFound = 'IdentifierNotFound',
  /**
   */
  InvalidRequest = 'InvalidRequest',
  /**
   */
  InconsistentData = 'InconsistentData',
  /**
   */
  Downcast = 'Downcast',
  /**
   */
  Deserialize = 'Deserialize',
  /**
   */
  Custom = 'Custom',
  /**
   */
  Io = 'Io',
}
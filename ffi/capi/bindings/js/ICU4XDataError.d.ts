
/**

 * Additional information: {@link https://docs.rs/icu/latest/icu/provider/struct.DataError.html 1}, {@link https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html 2}
 */
export enum ICU4XDataError {
  /**
   */
  Unknown = 'Unknown',
  /**
   */
  MissingDataMarker = 'MissingDataMarker',
  /**
   */
  MissingVariant = 'MissingVariant',
  /**
   */
  MissingLocale = 'MissingLocale',
  /**
   */
  NeedsVariant = 'NeedsVariant',
  /**
   */
  NeedsLocale = 'NeedsLocale',
  /**
   */
  ExtraneousLocale = 'ExtraneousLocale',
  /**
   */
  FilteredResource = 'FilteredResource',
  /**
   */
  MismatchedType = 'MismatchedType',
  /**
   */
  InvalidState = 'InvalidState',
  /**
   */
  Custom = 'Custom',
  /**
   */
  Io = 'Io',
  /**
   */
  UnavailableBufferFormat = 'UnavailableBufferFormat',
  /**
   */
  MismatchedAnyBuffer = 'MismatchedAnyBuffer',
  /**
   */
  DataStructValidityError = 'DataStructValidityError',
}
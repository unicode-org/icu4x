#ifndef ICU4XError_HPP
#define ICU4XError_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XError.h"
}



/**
 * A common enum for errors that ICU4X may return, organized by API
 * 
 * The error names are stable and can be checked against as strings in the JS API
 */
enum struct ICU4XError {

  /**
   * The error is not currently categorized as ICU4XError.
   * Please file a bug
   */
  UnknownError = 0,

  /**
   * An error arising from writing to a string
   * Typically found when not enough space is allocated
   * Most APIs that return a string may return this error
   */
  WriteableError = 1,
  OutOfBoundsError = 2,
  DataMissingResourceKeyError = 256,
  DataMissingVariantError = 257,
  DataMissingLocaleError = 258,
  DataMissingResourceOptionsError = 259,
  DataNeedsVariantError = 260,
  DataNeedsLocaleError = 261,
  DataExtraneousResourceOptionsError = 262,
  DataFilteredResourceError = 263,
  DataMismatchedTypeError = 264,
  DataMissingPayloadError = 265,
  DataInvalidStateError = 266,
  DataCustomError = 267,
  DataIoError = 268,
  DataUnavailableBufferFormatError = 269,

  /**
   * The subtag being requested was not set
   */
  LocaleUndefinedSubtagError = 512,

  /**
   * The locale or subtag string failed to parse
   */
  LocaleParserError = 513,

  /**
   * Attempted to construct an invalid data struct
   */
  DataStructValidityError = 768,
  PropertyUnknownScriptIdError = 1024,
  PropertyUnknownGeneralCategoryGroupError = 1025,
  DecimalLimitError = 1280,
  DecimalSyntaxError = 1281,
  PluralParserError = 1536,
};

#endif

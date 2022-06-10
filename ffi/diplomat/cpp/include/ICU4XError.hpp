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
   */
  WriteableError = 1,

  /**
   * The subtag being requested was not set
   */
  LocaleUndefinedSubtagError = 2,

  /**
   * The locale or subtag string failed to parse
   */
  LocaleParserError = 3,

  /**
   * Attempted to construct an invalid data struct
   */
  DataStructValidityError = 4,
  DataMissingResourceKeyError = 5,
  DataMissingVariantError = 6,
  DataMissingLocaleError = 7,
  DataMissingResourceOptionsError = 8,
  DataNeedsVariantError = 9,
  DataNeedsLocaleError = 10,
  DataExtraneousResourceOptionsError = 11,
  DataFilteredResourceError = 12,
  DataMismatchedTypeError = 13,
  DataMissingPayloadError = 14,
  DataInvalidStateError = 15,
  DataCustomError = 16,
  DataIoError = 17,
  DataUnavailableBufferFormatError = 18,
};

#endif

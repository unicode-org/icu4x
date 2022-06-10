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
  DataMissingResourceKeyError = 10,
  DataMissingVariantError = 11,
  DataMissingLocaleError = 12,
  DataMissingResourceOptionsError = 13,
  DataNeedsVariantError = 14,
  DataNeedsLocaleError = 15,
  DataExtraneousResourceOptionsError = 16,
  DataFilteredResourceError = 17,
  DataMismatchedTypeError = 18,
  DataMissingPayloadError = 19,
  DataInvalidStateError = 20,
  DataCustomError = 21,
  DataIoError = 22,
  DataUnavailableBufferFormatError = 23,

  /**
   * The subtag being requested was not set
   */
  LocaleUndefinedSubtagError = 31,

  /**
   * The locale or subtag string failed to parse
   */
  LocaleParserError = 32,

  /**
   * Attempted to construct an invalid data struct
   */
  DataStructValidityError = 33,
  PropertyUnknownScriptIdError = 40,
  PropertyUnknownGeneralCategoryGroupError = 41,
};

#endif

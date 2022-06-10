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
};

#endif

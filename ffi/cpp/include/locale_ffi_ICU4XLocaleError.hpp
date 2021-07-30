#ifndef locale_ffi_ICU4XLocaleError_HPP
#define locale_ffi_ICU4XLocaleError_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "locale_ffi_ICU4XLocaleError.h"
}


enum struct ICU4XLocaleError {
  Undefined = 0,
  Error = 1,
};

#endif

#ifndef ICU4XCanonicalizationResult_HPP
#define ICU4XCanonicalizationResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCanonicalizationResult.h"
}



/**
 * FFI version of `CanonicalizationResult`.
 * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/enum.CanonicalizationResult.html) for more details.
 */
enum struct ICU4XCanonicalizationResult {
  Modified = 0,
  Unmodified = 1,
};

#endif

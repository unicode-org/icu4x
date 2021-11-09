#ifndef ICU4XUnicodeSetPropertyResult_HPP
#define ICU4XUnicodeSetPropertyResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include <span>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XUnicodeSetPropertyResult.h"
}

class ICU4XUnicodeSetProperty;

/**
 * A destruction policy for using ICU4XUnicodeSetPropertyResult with std::unique_ptr.
 */
struct ICU4XUnicodeSetPropertyResultDeleter {
  void operator()(capi::ICU4XUnicodeSetPropertyResult* l) const noexcept {
    capi::ICU4XUnicodeSetPropertyResult_destroy(l);
  }
};
struct ICU4XUnicodeSetPropertyResult {
 public:

  /**
   * The [`ICU4XUnicodeSetProperty`], if creation was successful.
   */
  std::optional<ICU4XUnicodeSetProperty> data;

  /**
   * Whether creating the [`ICU4XUnicodeSetProperty`] was successful.
   */
  bool success;
};


#endif

#ifndef ICU4XUnicodeScriptMapPropertyResult_HPP
#define ICU4XUnicodeScriptMapPropertyResult_HPP
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
#include "ICU4XUnicodeScriptMapPropertyResult.h"
}

class ICU4XUnicodeScriptMapProperty;

/**
 * A destruction policy for using ICU4XUnicodeScriptMapPropertyResult with std::unique_ptr.
 */
struct ICU4XUnicodeScriptMapPropertyResultDeleter {
  void operator()(capi::ICU4XUnicodeScriptMapPropertyResult* l) const noexcept {
    capi::ICU4XUnicodeScriptMapPropertyResult_destroy(l);
  }
};
struct ICU4XUnicodeScriptMapPropertyResult {
 public:

  /**
   * The [`ICU4XUnicodeScriptMapProperty`], if creation was successful.
   */
  std::optional<ICU4XUnicodeScriptMapProperty> data;

  /**
   * Whether creating the [`ICU4XUnicodeScriptMapProperty`] was successful.
   */
  bool success;
};


#endif

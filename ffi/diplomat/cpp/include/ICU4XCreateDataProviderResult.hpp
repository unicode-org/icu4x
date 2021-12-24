#ifndef ICU4XCreateDataProviderResult_HPP
#define ICU4XCreateDataProviderResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreateDataProviderResult.h"
}

class ICU4XDataProvider;

/**
 * A destruction policy for using ICU4XCreateDataProviderResult with std::unique_ptr.
 */
struct ICU4XCreateDataProviderResultDeleter {
  void operator()(capi::ICU4XCreateDataProviderResult* l) const noexcept {
    capi::ICU4XCreateDataProviderResult_destroy(l);
  }
};

/**
 * A result type for `ICU4XDataProvider::create`.
 */
struct ICU4XCreateDataProviderResult {
 public:

  /**
   * Will be `None` if `success` is `false`, do not use in that case.
   */
  std::optional<ICU4XDataProvider> provider;
  bool success;
};


#endif

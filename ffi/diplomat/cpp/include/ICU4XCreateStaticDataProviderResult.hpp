#ifndef ICU4XCreateStaticDataProviderResult_HPP
#define ICU4XCreateStaticDataProviderResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <span>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreateStaticDataProviderResult.h"
}

class ICU4XStaticDataProvider;

/**
 * A destruction policy for using ICU4XCreateStaticDataProviderResult with std::unique_ptr.
 */
struct ICU4XCreateStaticDataProviderResultDeleter {
  void operator()(capi::ICU4XCreateStaticDataProviderResult* l) const noexcept {
    capi::ICU4XCreateStaticDataProviderResult_destroy(l);
  }
};

/**
 * A result type for `ICU4XStaticDataProvider::create`.
 */
struct ICU4XCreateStaticDataProviderResult {
 public:

  /**
   * Will be `None` if `success` is `false`, do not use in that case.
   */
  std::optional<ICU4XStaticDataProvider> provider;
  bool success;
};


#endif

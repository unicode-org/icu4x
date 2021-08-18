#ifndef ICU4XCreateStaticDataProviderResult_HPP
#define ICU4XCreateStaticDataProviderResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreateStaticDataProviderResult.h"
}

class ICU4XStaticDataProvider;

struct ICU4XCreateStaticDataProviderResultDeleter {
  void operator()(capi::ICU4XCreateStaticDataProviderResult* l) const noexcept {
    capi::ICU4XCreateStaticDataProviderResult_destroy(l);
  }
};
struct ICU4XCreateStaticDataProviderResult {
 public:
  std::optional<ICU4XStaticDataProvider> provider;
  bool success;
};


#endif

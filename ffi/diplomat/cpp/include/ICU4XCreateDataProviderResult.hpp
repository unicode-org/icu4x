#ifndef ICU4XCreateDataProviderResult_HPP
#define ICU4XCreateDataProviderResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreateDataProviderResult.h"
}

class ICU4XDataProvider;

struct ICU4XCreateDataProviderResultDeleter {
  void operator()(capi::ICU4XCreateDataProviderResult* l) const noexcept {
    capi::ICU4XCreateDataProviderResult_destroy(l);
  }
};
struct ICU4XCreateDataProviderResult {
 public:
  std::optional<ICU4XDataProvider> provider;
  bool success;
};


#endif

#ifndef ICU4XCreateFixedDecimalFormatDataProviderResult_HPP
#define ICU4XCreateFixedDecimalFormatDataProviderResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreateFixedDecimalFormatDataProviderResult.h"
}

class ICU4XFixedDecimalFormatDataProvider;

struct ICU4XCreateFixedDecimalFormatDataProviderResultDeleter {
  void operator()(capi::ICU4XCreateFixedDecimalFormatDataProviderResult* l) const noexcept {
    capi::ICU4XCreateFixedDecimalFormatDataProviderResult_destroy(l);
  }
};
struct ICU4XCreateFixedDecimalFormatDataProviderResult {
 public:
  std::optional<ICU4XFixedDecimalFormatDataProvider> provider;
  bool success;
};


#endif

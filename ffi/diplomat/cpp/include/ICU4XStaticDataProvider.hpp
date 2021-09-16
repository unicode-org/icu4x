#ifndef ICU4XStaticDataProvider_HPP
#define ICU4XStaticDataProvider_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XStaticDataProvider.h"
}

struct ICU4XCreateStaticDataProviderResult;

struct ICU4XStaticDataProviderDeleter {
  void operator()(capi::ICU4XStaticDataProvider* l) const noexcept {
    capi::ICU4XStaticDataProvider_destroy(l);
  }
};
class ICU4XStaticDataProvider {
 public:
  static ICU4XCreateStaticDataProviderResult create();
  inline const capi::ICU4XStaticDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XStaticDataProvider* AsFFIMut() { return this->inner.get(); }
  inline ICU4XStaticDataProvider(capi::ICU4XStaticDataProvider* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XStaticDataProvider, ICU4XStaticDataProviderDeleter> inner;
};

#include "ICU4XCreateStaticDataProviderResult.hpp"

inline ICU4XCreateStaticDataProviderResult ICU4XStaticDataProvider::create() {
  capi::ICU4XCreateStaticDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XStaticDataProvider_create();
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XStaticDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XStaticDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateStaticDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}
#endif

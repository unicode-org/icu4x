#ifndef ICU4XFixedDecimalFormatDataProvider_HPP
#define ICU4XFixedDecimalFormatDataProvider_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalFormatDataProvider.h"
}

struct ICU4XCreateFixedDecimalFormatDataProviderResult;

struct ICU4XFixedDecimalFormatDataProviderDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatDataProvider* l) const noexcept {
    capi::ICU4XFixedDecimalFormatDataProvider_destroy(l);
  }
};
class ICU4XFixedDecimalFormatDataProvider {
 public:
  static ICU4XCreateFixedDecimalFormatDataProviderResult create_static();
  inline const capi::ICU4XFixedDecimalFormatDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimalFormatDataProvider* AsFFIMut() { return this->inner.get(); }
  inline ICU4XFixedDecimalFormatDataProvider(capi::ICU4XFixedDecimalFormatDataProvider* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XFixedDecimalFormatDataProvider, ICU4XFixedDecimalFormatDataProviderDeleter> inner;
};

#include "ICU4XCreateFixedDecimalFormatDataProviderResult.hpp"

inline ICU4XCreateFixedDecimalFormatDataProviderResult ICU4XFixedDecimalFormatDataProvider::create_static() {
  capi::ICU4XCreateFixedDecimalFormatDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XFixedDecimalFormatDataProvider_create_static();
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XFixedDecimalFormatDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XFixedDecimalFormatDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateFixedDecimalFormatDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}
#endif

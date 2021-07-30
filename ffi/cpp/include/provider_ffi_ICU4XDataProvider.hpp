#ifndef provider_ffi_ICU4XDataProvider_HPP
#define provider_ffi_ICU4XDataProvider_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "provider_ffi_ICU4XDataProvider.h"
}

struct ICU4XCreateDataProviderResult;

struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};
class ICU4XDataProvider {
 public:
  static ICU4XCreateDataProviderResult create_fs(const std::string_view path);
  static ICU4XCreateDataProviderResult create_static();
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDataProvider* AsFFIMut() { return this->inner.get(); }
  ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};

#include "provider_ffi_ICU4XCreateDataProviderResult.hpp"

ICU4XCreateDataProviderResult ICU4XDataProvider::create_fs(const std::string_view path) {
  capi::ICU4XCreateDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XDataProvider_create_fs(path.data(), path.length());
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}
ICU4XCreateDataProviderResult ICU4XDataProvider::create_static() {
  capi::ICU4XCreateDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XDataProvider_create_static();
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}
#endif

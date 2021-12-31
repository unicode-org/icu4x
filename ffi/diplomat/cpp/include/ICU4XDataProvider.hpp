#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XDataProvider.h"
}

struct ICU4XCreateDataProviderResult;

/**
 * A destruction policy for using ICU4XDataProvider with std::unique_ptr.
 */
struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};
class ICU4XDataProvider {
 public:

  /**
   * Constructs an `FsDataProvider` and returns it as an [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html) for more details.
   */
  static ICU4XCreateDataProviderResult create_fs(const std::string_view path);

  /**
   * Constructs an `StaticDataProvider` and returns it as an [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html) for more details.
   */
  static ICU4XCreateDataProviderResult create_static();

  /**
   * Constructs a `BlobDataProvider` and returns it as an [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html) for more details.
   */
  static ICU4XCreateDataProviderResult create_from_byte_slice(const diplomat::span<uint8_t> blob);

  /**
   * Constructs an empty `StaticDataProvider` and returns it as an [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html) for more details.
   */
  static ICU4XCreateDataProviderResult create_empty();
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDataProvider* AsFFIMut() { return this->inner.get(); }
  inline ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
  ICU4XDataProvider() = default;
  ICU4XDataProvider(ICU4XDataProvider&&) noexcept = default;
  ICU4XDataProvider& operator=(ICU4XDataProvider&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};

#include "ICU4XCreateDataProviderResult.hpp"

inline ICU4XCreateDataProviderResult ICU4XDataProvider::create_fs(const std::string_view path) {
  capi::ICU4XCreateDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XDataProvider_create_fs(path.data(), path.size());
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline ICU4XCreateDataProviderResult ICU4XDataProvider::create_static() {
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
inline ICU4XCreateDataProviderResult ICU4XDataProvider::create_from_byte_slice(const diplomat::span<uint8_t> blob) {
  capi::ICU4XCreateDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XDataProvider_create_from_byte_slice(blob.data(), blob.size());
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline ICU4XCreateDataProviderResult ICU4XDataProvider::create_empty() {
  capi::ICU4XCreateDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XDataProvider_create_empty();
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

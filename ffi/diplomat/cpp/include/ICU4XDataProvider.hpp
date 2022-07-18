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

#include "ICU4XDataProvider.h"

class ICU4XDataProvider;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XDataProvider with std::unique_ptr.
 */
struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};

/**
 * An ICU4X data provider, capable of loading ICU4X data keys from some source.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html) for more information.
 */
class ICU4XDataProvider {
 public:

  /**
   * Constructs an `FsDataProvider` and returns it as an [`ICU4XDataProvider`].
   * Requires the `provider_fs` feature.
   * Not supported in WASM.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html) for more information.
   */
  static diplomat::result<ICU4XDataProvider, ICU4XError> create_fs(const std::string_view path);

  /**
   * Constructs a testdata provider and returns it as an [`ICU4XDataProvider`].
   * Requires the `provider_test` feature.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/index.html) for more information.
   */
  static ICU4XDataProvider create_test();

  /**
   * Constructs a `BlobDataProvider` and returns it as an [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html) for more information.
   */
  static diplomat::result<ICU4XDataProvider, ICU4XError> create_from_byte_slice(const diplomat::span<uint8_t> blob);

  /**
   * Constructs an empty `StaticDataProvider` and returns it as an [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html) for more information.
   */
  static ICU4XDataProvider create_empty();
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDataProvider* AsFFIMut() { return this->inner.get(); }
  inline ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
  ICU4XDataProvider() = default;
  ICU4XDataProvider(ICU4XDataProvider&&) noexcept = default;
  ICU4XDataProvider& operator=(ICU4XDataProvider&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};


inline diplomat::result<ICU4XDataProvider, ICU4XError> ICU4XDataProvider::create_fs(const std::string_view path) {
  auto diplomat_result_raw_out_value = capi::ICU4XDataProvider_create_fs(path.data(), path.size());
  diplomat::result<ICU4XDataProvider, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XDataProvider>(std::move(ICU4XDataProvider(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XDataProvider ICU4XDataProvider::create_test() {
  return ICU4XDataProvider(capi::ICU4XDataProvider_create_test());
}
inline diplomat::result<ICU4XDataProvider, ICU4XError> ICU4XDataProvider::create_from_byte_slice(const diplomat::span<uint8_t> blob) {
  auto diplomat_result_raw_out_value = capi::ICU4XDataProvider_create_from_byte_slice(blob.data(), blob.size());
  diplomat::result<ICU4XDataProvider, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XDataProvider>(std::move(ICU4XDataProvider(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XDataProvider ICU4XDataProvider::create_empty() {
  return ICU4XDataProvider(capi::ICU4XDataProvider_create_empty());
}
#endif

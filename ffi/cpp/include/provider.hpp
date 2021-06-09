// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_PROVIDER_HPP
#define ICU4X_PROVIDER_HPP

#include <algorithm>
#include <optional>
#include <string_view>

#include "../../capi/include/provider.h"

namespace icu4x {
class DataProvider {
 public:
  ~DataProvider() {
    // Check that the fat pointer not nulled out
    if (this->inner._field1 && this->inner._field2) {
      icu4x_data_provider_destroy(this->inner);
    }
  }
  DataProvider(const DataProvider&) = delete;
  DataProvider& operator=(const DataProvider&) = delete;
  DataProvider(DataProvider&& other) noexcept {
    this->inner = other.inner;
    // Null out the fat pointer
    other.inner = {0, 0};
  }
  DataProvider& operator=(DataProvider&& other) noexcept {
    std::swap(inner, other.inner);
    return *this;
  }
  static std::optional<DataProvider> FsDataProvider(
      const std::string_view& path) {
    ICU4XCreateDataProviderResult result =
        icu4x_fs_data_provider_create(path.data(), path.size());
    if (!result.success) {
      return {};
    }
    return DataProvider(result.provider);
  }
  inline ICU4XDataProvider AsFFI() const { return this->inner; }

 private:
  DataProvider(ICU4XDataProvider i) : inner(i) {}
  ICU4XDataProvider inner;
};
}  // namespace icu4x

#endif  // ICU4X_PROVIDER_HPP
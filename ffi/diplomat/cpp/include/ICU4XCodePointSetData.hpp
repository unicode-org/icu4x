#ifndef ICU4XCodePointSetData_HPP
#define ICU4XCodePointSetData_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCodePointSetData.h"
}

class ICU4XDataProvider;
struct ICU4XCodePointSetDataResult;

/**
 * A destruction policy for using ICU4XCodePointSetData with std::unique_ptr.
 */
struct ICU4XCodePointSetDataDeleter {
  void operator()(capi::ICU4XCodePointSetData* l) const noexcept {
    capi::ICU4XCodePointSetData_destroy(l);
  }
};
class ICU4XCodePointSetData {
 public:

  /**
   * Gets a set for Unicode property ascii_hex_digit from a [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html) for more information.
   */
  static ICU4XCodePointSetDataResult try_get_ascii_hex_digit(const ICU4XDataProvider& provider);

  /**
   * Checks whether the code point is in the set.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_uniset/struct.UnicodeSet.html#method.contains) for more information.
   */
  bool contains(char32_t cp) const;
  inline const capi::ICU4XCodePointSetData* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCodePointSetData* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCodePointSetData(capi::ICU4XCodePointSetData* i) : inner(i) {}
  ICU4XCodePointSetData() = default;
  ICU4XCodePointSetData(ICU4XCodePointSetData&&) noexcept = default;
  ICU4XCodePointSetData& operator=(ICU4XCodePointSetData&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCodePointSetData, ICU4XCodePointSetDataDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XCodePointSetDataResult.hpp"

inline ICU4XCodePointSetDataResult ICU4XCodePointSetData::try_get_ascii_hex_digit(const ICU4XDataProvider& provider) {
  capi::ICU4XCodePointSetDataResult diplomat_raw_struct_out_value = capi::ICU4XCodePointSetData_try_get_ascii_hex_digit(provider.AsFFI());
  auto diplomat_optional_raw_out_value_data = diplomat_raw_struct_out_value.data;
  std::optional<ICU4XCodePointSetData> diplomat_optional_out_value_data;
  if (diplomat_optional_raw_out_value_data != nullptr) {
    diplomat_optional_out_value_data = ICU4XCodePointSetData(diplomat_optional_raw_out_value_data);
  } else {
    diplomat_optional_out_value_data = std::nullopt;
  }
  return ICU4XCodePointSetDataResult{ .data = std::move(diplomat_optional_out_value_data), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline bool ICU4XCodePointSetData::contains(char32_t cp) const {
  return capi::ICU4XCodePointSetData_contains(this->inner.get(), cp);
}
#endif

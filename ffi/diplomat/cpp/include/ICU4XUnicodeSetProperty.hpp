#ifndef ICU4XUnicodeSetProperty_HPP
#define ICU4XUnicodeSetProperty_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include <span>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XUnicodeSetProperty.h"
}

class ICU4XDataProvider;
struct ICU4XUnicodeSetPropertyResult;
class ICU4XStaticDataProvider;

/**
 * A destruction policy for using ICU4XUnicodeSetProperty with std::unique_ptr.
 */
struct ICU4XUnicodeSetPropertyDeleter {
  void operator()(capi::ICU4XUnicodeSetProperty* l) const noexcept {
    capi::ICU4XUnicodeSetProperty_destroy(l);
  }
};
class ICU4XUnicodeSetProperty {
 public:

  /**
   * Gets a set for Unicode property ascii_hex_digit from a [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html) for more information.
   */
  static ICU4XUnicodeSetPropertyResult try_get_ascii_hex_digit(const ICU4XDataProvider& provider);

  /**
   * Gets a set for Unicode property ascii_hex_digit from a [`ICU4XStaticDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html) for more information.
   */
  static ICU4XUnicodeSetPropertyResult try_get_ascii_hex_digit_from_static(const ICU4XStaticDataProvider& provider);

  /**
   * Checks whether the code point is in the set.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_uniset/struct.UnicodeSet.html#method.contains) for more information.
   */
  bool contains(char32_t cp) const;
  inline const capi::ICU4XUnicodeSetProperty* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XUnicodeSetProperty* AsFFIMut() { return this->inner.get(); }
  inline ICU4XUnicodeSetProperty(capi::ICU4XUnicodeSetProperty* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XUnicodeSetProperty, ICU4XUnicodeSetPropertyDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XUnicodeSetPropertyResult.hpp"
#include "ICU4XStaticDataProvider.hpp"

inline ICU4XUnicodeSetPropertyResult ICU4XUnicodeSetProperty::try_get_ascii_hex_digit(const ICU4XDataProvider& provider) {
  capi::ICU4XUnicodeSetPropertyResult diplomat_raw_struct_out_value = capi::ICU4XUnicodeSetProperty_try_get_ascii_hex_digit(provider.AsFFI());
  auto diplomat_optional_raw_out_value_data = diplomat_raw_struct_out_value.data;
  std::optional<ICU4XUnicodeSetProperty> diplomat_optional_out_value_data;
  if (diplomat_optional_raw_out_value_data != nullptr) {
    diplomat_optional_out_value_data = ICU4XUnicodeSetProperty(diplomat_optional_raw_out_value_data);
  } else {
    diplomat_optional_out_value_data = std::nullopt;
  }
  return ICU4XUnicodeSetPropertyResult{ .data = std::move(diplomat_optional_out_value_data), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline ICU4XUnicodeSetPropertyResult ICU4XUnicodeSetProperty::try_get_ascii_hex_digit_from_static(const ICU4XStaticDataProvider& provider) {
  capi::ICU4XUnicodeSetPropertyResult diplomat_raw_struct_out_value = capi::ICU4XUnicodeSetProperty_try_get_ascii_hex_digit_from_static(provider.AsFFI());
  auto diplomat_optional_raw_out_value_data = diplomat_raw_struct_out_value.data;
  std::optional<ICU4XUnicodeSetProperty> diplomat_optional_out_value_data;
  if (diplomat_optional_raw_out_value_data != nullptr) {
    diplomat_optional_out_value_data = ICU4XUnicodeSetProperty(diplomat_optional_raw_out_value_data);
  } else {
    diplomat_optional_out_value_data = std::nullopt;
  }
  return ICU4XUnicodeSetPropertyResult{ .data = std::move(diplomat_optional_out_value_data), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline bool ICU4XUnicodeSetProperty::contains(char32_t cp) const {
  return capi::ICU4XUnicodeSetProperty_contains(this->inner.get(), cp);
}
#endif

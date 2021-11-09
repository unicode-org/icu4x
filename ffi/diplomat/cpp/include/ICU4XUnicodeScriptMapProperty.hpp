#ifndef ICU4XUnicodeScriptMapProperty_HPP
#define ICU4XUnicodeScriptMapProperty_HPP
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
#include "ICU4XUnicodeScriptMapProperty.h"
}

class ICU4XDataProvider;
struct ICU4XUnicodeScriptMapPropertyResult;
class ICU4XStaticDataProvider;

/**
 * A destruction policy for using ICU4XUnicodeScriptMapProperty with std::unique_ptr.
 */
struct ICU4XUnicodeScriptMapPropertyDeleter {
  void operator()(capi::ICU4XUnicodeScriptMapProperty* l) const noexcept {
    capi::ICU4XUnicodeScriptMapProperty_destroy(l);
  }
};
class ICU4XUnicodeScriptMapProperty {
 public:

  /**
   * Gets a set for Unicode property ascii_hex_digit from a [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html) for more information.
   */
  static ICU4XUnicodeScriptMapPropertyResult try_get(const ICU4XDataProvider& provider);

  /**
   * Gets a set for Unicode property ascii_hex_digit from a [`ICU4XStaticDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html) for more information.
   */
  static ICU4XUnicodeScriptMapPropertyResult try_get_from_static(const ICU4XStaticDataProvider& provider);

  /**
   * Gets the Script for a code point.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32) for more information.
   */
  uint32_t get(char32_t cp) const;
  inline const capi::ICU4XUnicodeScriptMapProperty* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XUnicodeScriptMapProperty* AsFFIMut() { return this->inner.get(); }
  inline ICU4XUnicodeScriptMapProperty(capi::ICU4XUnicodeScriptMapProperty* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XUnicodeScriptMapProperty, ICU4XUnicodeScriptMapPropertyDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XUnicodeScriptMapPropertyResult.hpp"
#include "ICU4XStaticDataProvider.hpp"

inline ICU4XUnicodeScriptMapPropertyResult ICU4XUnicodeScriptMapProperty::try_get(const ICU4XDataProvider& provider) {
  capi::ICU4XUnicodeScriptMapPropertyResult diplomat_raw_struct_out_value = capi::ICU4XUnicodeScriptMapProperty_try_get(provider.AsFFI());
  auto diplomat_optional_raw_out_value_data = diplomat_raw_struct_out_value.data;
  std::optional<ICU4XUnicodeScriptMapProperty> diplomat_optional_out_value_data;
  if (diplomat_optional_raw_out_value_data != nullptr) {
    diplomat_optional_out_value_data = ICU4XUnicodeScriptMapProperty(diplomat_optional_raw_out_value_data);
  } else {
    diplomat_optional_out_value_data = std::nullopt;
  }
  return ICU4XUnicodeScriptMapPropertyResult{ .data = std::move(diplomat_optional_out_value_data), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline ICU4XUnicodeScriptMapPropertyResult ICU4XUnicodeScriptMapProperty::try_get_from_static(const ICU4XStaticDataProvider& provider) {
  capi::ICU4XUnicodeScriptMapPropertyResult diplomat_raw_struct_out_value = capi::ICU4XUnicodeScriptMapProperty_try_get_from_static(provider.AsFFI());
  auto diplomat_optional_raw_out_value_data = diplomat_raw_struct_out_value.data;
  std::optional<ICU4XUnicodeScriptMapProperty> diplomat_optional_out_value_data;
  if (diplomat_optional_raw_out_value_data != nullptr) {
    diplomat_optional_out_value_data = ICU4XUnicodeScriptMapProperty(diplomat_optional_raw_out_value_data);
  } else {
    diplomat_optional_out_value_data = std::nullopt;
  }
  return ICU4XUnicodeScriptMapPropertyResult{ .data = std::move(diplomat_optional_out_value_data), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline uint32_t ICU4XUnicodeScriptMapProperty::get(char32_t cp) const {
  return capi::ICU4XUnicodeScriptMapProperty_get(this->inner.get(), cp);
}
#endif

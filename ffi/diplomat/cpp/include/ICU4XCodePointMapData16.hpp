#ifndef ICU4XCodePointMapData16_HPP
#define ICU4XCodePointMapData16_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCodePointMapData16.h"
}

class ICU4XDataProvider;
struct ICU4XCodePointMapData16Response;

/**
 * A destruction policy for using ICU4XCodePointMapData16 with std::unique_ptr.
 */
struct ICU4XCodePointMapData16Deleter {
  void operator()(capi::ICU4XCodePointMapData16* l) const noexcept {
    capi::ICU4XCodePointMapData16_destroy(l);
  }
};
class ICU4XCodePointMapData16 {
 public:

  /**
   * Gets a map for Unicode property Script from a [`ICU4XDataProvider`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html) for more information.
   */
  static ICU4XCodePointMapData16Response try_get_script(const ICU4XDataProvider& provider);

  /**
   * Gets the value for a code point.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32) for more information.
   */
  uint16_t get(char32_t cp) const;
  inline const capi::ICU4XCodePointMapData16* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCodePointMapData16* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCodePointMapData16(capi::ICU4XCodePointMapData16* i) : inner(i) {}
  ICU4XCodePointMapData16() = default;
  ICU4XCodePointMapData16(ICU4XCodePointMapData16&&) noexcept = default;
  ICU4XCodePointMapData16& operator=(ICU4XCodePointMapData16&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCodePointMapData16, ICU4XCodePointMapData16Deleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XCodePointMapData16Response.hpp"

inline ICU4XCodePointMapData16Response ICU4XCodePointMapData16::try_get_script(const ICU4XDataProvider& provider) {
  capi::ICU4XCodePointMapData16Response diplomat_raw_struct_out_value = capi::ICU4XCodePointMapData16_try_get_script(provider.AsFFI());
  auto diplomat_optional_raw_out_value_data = diplomat_raw_struct_out_value.data;
  std::optional<ICU4XCodePointMapData16> diplomat_optional_out_value_data;
  if (diplomat_optional_raw_out_value_data != nullptr) {
    diplomat_optional_out_value_data = ICU4XCodePointMapData16(diplomat_optional_raw_out_value_data);
  } else {
    diplomat_optional_out_value_data = std::nullopt;
  }
  return ICU4XCodePointMapData16Response{ .data = std::move(diplomat_optional_out_value_data), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline uint16_t ICU4XCodePointMapData16::get(char32_t cp) const {
  return capi::ICU4XCodePointMapData16_get(this->inner.get(), cp);
}
#endif

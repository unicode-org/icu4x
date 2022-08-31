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

#include "ICU4XCodePointMapData16.h"

class ICU4XCodePointSetData;
class ICU4XDataProvider;
class ICU4XCodePointMapData16;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XCodePointMapData16 with std::unique_ptr.
 */
struct ICU4XCodePointMapData16Deleter {
  void operator()(capi::ICU4XCodePointMapData16* l) const noexcept {
    capi::ICU4XCodePointMapData16_destroy(l);
  }
};

/**
 * An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
 * 
 * For properties whose values fit into 16 bits.
 * 
 * See the [Rust documentation for `properties`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html) for more information.
 * 
 * See the [Rust documentation for `CodePointMapData`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapData.html) for more information.
 * 
 * See the [Rust documentation for `CodePointMapDataBorrowed`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html) for more information.
 */
class ICU4XCodePointMapData16 {
 public:

  /**
   * Gets the value for a code point.
   * 
   * See the [Rust documentation for `get`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get) for more information.
   */
  uint16_t get(char32_t cp) const;

  /**
   * Gets the value for a code point (specified as a 32 bit integer, in UTF-32)
   */
  uint16_t get_u32(uint32_t cp) const;

  /**
   * Gets a [`ICU4XCodePointSetData`] representing all entries in this map that map to the given value
   * 
   * See the [Rust documentation for `get_set_for_value`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value) for more information.
   */
  ICU4XCodePointSetData get_set_for_value(uint16_t value) const;

  /**
   * 
   * 
   * See the [Rust documentation for `load_script`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_script.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData16, ICU4XError> load_script(const ICU4XDataProvider& provider);
  inline const capi::ICU4XCodePointMapData16* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCodePointMapData16* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCodePointMapData16(capi::ICU4XCodePointMapData16* i) : inner(i) {}
  ICU4XCodePointMapData16() = default;
  ICU4XCodePointMapData16(ICU4XCodePointMapData16&&) noexcept = default;
  ICU4XCodePointMapData16& operator=(ICU4XCodePointMapData16&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCodePointMapData16, ICU4XCodePointMapData16Deleter> inner;
};

#include "ICU4XCodePointSetData.hpp"
#include "ICU4XDataProvider.hpp"

inline uint16_t ICU4XCodePointMapData16::get(char32_t cp) const {
  return capi::ICU4XCodePointMapData16_get(this->inner.get(), cp);
}
inline uint16_t ICU4XCodePointMapData16::get_u32(uint32_t cp) const {
  return capi::ICU4XCodePointMapData16_get_u32(this->inner.get(), cp);
}
inline ICU4XCodePointSetData ICU4XCodePointMapData16::get_set_for_value(uint16_t value) const {
  return ICU4XCodePointSetData(capi::ICU4XCodePointMapData16_get_set_for_value(this->inner.get(), value));
}
inline diplomat::result<ICU4XCodePointMapData16, ICU4XError> ICU4XCodePointMapData16::load_script(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData16_load_script(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData16, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData16>(std::move(ICU4XCodePointMapData16(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif

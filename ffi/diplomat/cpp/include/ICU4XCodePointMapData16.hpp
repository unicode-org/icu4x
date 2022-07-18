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
 * An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. For properties whose values fit into 16 bits.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html) for more information.
 */
class ICU4XCodePointMapData16 {
 public:

  /**
   * Gets a map for Unicode property Script from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData16, ICU4XError> try_get_script(const ICU4XDataProvider& provider);

  /**
   * Gets the value for a code point.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32) for more information.
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

inline diplomat::result<ICU4XCodePointMapData16, ICU4XError> ICU4XCodePointMapData16::try_get_script(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData16_try_get_script(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData16, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData16>(std::move(ICU4XCodePointMapData16(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline uint16_t ICU4XCodePointMapData16::get(char32_t cp) const {
  return capi::ICU4XCodePointMapData16_get(this->inner.get(), cp);
}
#endif

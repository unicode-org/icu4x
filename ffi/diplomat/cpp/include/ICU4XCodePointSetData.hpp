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
class ICU4XCodePointSetData;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XCodePointSetData with std::unique_ptr.
 */
struct ICU4XCodePointSetDataDeleter {
  void operator()(capi::ICU4XCodePointSetData* l) const noexcept {
    capi::ICU4XCodePointSetData_destroy(l);
  }
};

/**
 * An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html) for more information.
 */
class ICU4XCodePointSetData {
 public:

  /**
   * Gets a set for Unicode property ascii_hex_digit from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> try_get_ascii_hex_digit(const ICU4XDataProvider& provider);

  /**
   * Checks whether the code point is in the set.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_uniset/struct.UnicodeSet.html#method.contains) for more information.
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

inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::try_get_ascii_hex_digit(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_try_get_ascii_hex_digit(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline bool ICU4XCodePointSetData::contains(char32_t cp) const {
  return capi::ICU4XCodePointSetData_contains(this->inner.get(), cp);
}
#endif

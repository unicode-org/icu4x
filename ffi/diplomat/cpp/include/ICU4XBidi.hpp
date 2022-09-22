#ifndef ICU4XBidi_HPP
#define ICU4XBidi_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XBidi.h"

class ICU4XDataProvider;
class ICU4XBidi;
#include "ICU4XError.hpp"
class ICU4XBidiInfo;

/**
 * A destruction policy for using ICU4XBidi with std::unique_ptr.
 */
struct ICU4XBidiDeleter {
  void operator()(capi::ICU4XBidi* l) const noexcept {
    capi::ICU4XBidi_destroy(l);
  }
};

/**
 * An ICU4X Bidi object, containing loaded bidi data
 * 
 * See the [Rust documentation for `BidiClassAdapter`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html) for more information.
 */
class ICU4XBidi {
 public:

  /**
   * Creates a new [`ICU4XBidi`] from locale data.
   * 
   * See the [Rust documentation for `new`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html#method.new) for more information.
   */
  static diplomat::result<ICU4XBidi, ICU4XError> create(const ICU4XDataProvider& provider);

  /**
   * Use the data loaded in this object to process a string and calculate bidi information
   * 
   * Takes in a Level for the default level, if it is an invalid value it will default to LTR
   * 
   * See the [Rust documentation for `new_with_data_source`](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.BidiInfo.html#method.new_with_data_source) for more information.
   */
  ICU4XBidiInfo for_text(const std::string_view text, uint8_t default_level) const;

  /**
   * Check if a Level returned by level_at is an RTL level.
   * 
   * Invalid levels (numbers greater than 125) will be assumed LTR
   * 
   * See the [Rust documentation for `is_rtl`](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_rtl) for more information.
   */
  static bool level_is_rtl(uint8_t level);

  /**
   * Check if a Level returned by level_at is an LTR level.
   * 
   * Invalid levels (numbers greater than 125) will be assumed LTR
   * 
   * See the [Rust documentation for `is_ltr`](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_ltr) for more information.
   */
  static bool level_is_ltr(uint8_t level);

  /**
   * Get a basic RTL Level value
   * 
   * See the [Rust documentation for `rtl`](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.rtl) for more information.
   */
  static uint8_t level_rtl();

  /**
   * Get a simple LTR Level value
   * 
   * See the [Rust documentation for `ltr`](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.ltr) for more information.
   */
  static uint8_t level_ltr();
  inline const capi::ICU4XBidi* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XBidi* AsFFIMut() { return this->inner.get(); }
  inline ICU4XBidi(capi::ICU4XBidi* i) : inner(i) {}
  ICU4XBidi() = default;
  ICU4XBidi(ICU4XBidi&&) noexcept = default;
  ICU4XBidi& operator=(ICU4XBidi&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XBidi, ICU4XBidiDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XBidiInfo.hpp"

inline diplomat::result<ICU4XBidi, ICU4XError> ICU4XBidi::create(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XBidi_create(provider.AsFFI());
  diplomat::result<ICU4XBidi, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XBidi>(std::move(ICU4XBidi(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XBidiInfo ICU4XBidi::for_text(const std::string_view text, uint8_t default_level) const {
  return ICU4XBidiInfo(capi::ICU4XBidi_for_text(this->inner.get(), text.data(), text.size(), default_level));
}
inline bool ICU4XBidi::level_is_rtl(uint8_t level) {
  return capi::ICU4XBidi_level_is_rtl(level);
}
inline bool ICU4XBidi::level_is_ltr(uint8_t level) {
  return capi::ICU4XBidi_level_is_ltr(level);
}
inline uint8_t ICU4XBidi::level_rtl() {
  return capi::ICU4XBidi_level_rtl();
}
inline uint8_t ICU4XBidi::level_ltr() {
  return capi::ICU4XBidi_level_ltr();
}
#endif

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

namespace capi {
#include "ICU4XBidi.h"
}

class ICU4XDataProvider;
class ICU4XBidi;
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
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html) for more information.
 */
class ICU4XBidi {
 public:

  /**
   * Creates a new [`ICU4XBidi`] from locale data.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html#method.new) for more information.
   */
  static diplomat::result<ICU4XBidi, std::monostate> try_new(const ICU4XDataProvider& provider);

  /**
   * Use the data loaded in this object to process a string and calculate bidi information
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.BidiInfo.html#method.new_with_data_source) for more information.
   */
  ICU4XBidiInfo for_text(const std::string_view text) const;
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

inline diplomat::result<ICU4XBidi, std::monostate> ICU4XBidi::try_new(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XBidi_try_new(provider.AsFFI());
  diplomat::result<ICU4XBidi, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XBidi(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline ICU4XBidiInfo ICU4XBidi::for_text(const std::string_view text) const {
  return ICU4XBidiInfo(capi::ICU4XBidi_for_text(this->inner.get(), text.data(), text.size()));
}
#endif

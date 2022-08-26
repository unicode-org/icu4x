#ifndef ICU4XIsoDateTime_HPP
#define ICU4XIsoDateTime_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XIsoDateTime.h"

class ICU4XIsoDateTime;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XIsoDateTime with std::unique_ptr.
 */
struct ICU4XIsoDateTimeDeleter {
  void operator()(capi::ICU4XIsoDateTime* l) const noexcept {
    capi::ICU4XIsoDateTime_destroy(l);
  }
};

/**
 * An ICU4X DateTime object capable of containing a ISO-8601 date and time.
 * 
 * See the [Rust documentation for `DateTime`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html) for more information.
 */
class ICU4XIsoDateTime {
 public:

  /**
   * Creates a new [`ICU4XIsoDateTime`] from the specified date and time.
   * 
   * See the [Rust documentation for `new_gregorian_datetime`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime) for more information.
   */
  static diplomat::result<ICU4XIsoDateTime, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second);
  inline const capi::ICU4XIsoDateTime* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XIsoDateTime* AsFFIMut() { return this->inner.get(); }
  inline ICU4XIsoDateTime(capi::ICU4XIsoDateTime* i) : inner(i) {}
  ICU4XIsoDateTime() = default;
  ICU4XIsoDateTime(ICU4XIsoDateTime&&) noexcept = default;
  ICU4XIsoDateTime& operator=(ICU4XIsoDateTime&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XIsoDateTime, ICU4XIsoDateTimeDeleter> inner;
};


inline diplomat::result<ICU4XIsoDateTime, ICU4XError> ICU4XIsoDateTime::try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second) {
  auto diplomat_result_raw_out_value = capi::ICU4XIsoDateTime_try_new(year, month, day, hour, minute, second);
  diplomat::result<ICU4XIsoDateTime, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XIsoDateTime>(std::move(ICU4XIsoDateTime(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif

#ifndef ICU4XGregorianDateTime_HPP
#define ICU4XGregorianDateTime_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XGregorianDateTime.h"

class ICU4XGregorianDateTime;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XGregorianDateTime with std::unique_ptr.
 */
struct ICU4XGregorianDateTimeDeleter {
  void operator()(capi::ICU4XGregorianDateTime* l) const noexcept {
    capi::ICU4XGregorianDateTime_destroy(l);
  }
};

/**
 * An ICU4X DateTime object capable of containing a Gregorian date and time.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html) for more information.
 */
class ICU4XGregorianDateTime {
 public:

  /**
   * Creates a new [`ICU4XGregorianDateTime`] from the specified date and time.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime) for more information.
   */
  static diplomat::result<ICU4XGregorianDateTime, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second);
  inline const capi::ICU4XGregorianDateTime* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XGregorianDateTime* AsFFIMut() { return this->inner.get(); }
  inline ICU4XGregorianDateTime(capi::ICU4XGregorianDateTime* i) : inner(i) {}
  ICU4XGregorianDateTime() = default;
  ICU4XGregorianDateTime(ICU4XGregorianDateTime&&) noexcept = default;
  ICU4XGregorianDateTime& operator=(ICU4XGregorianDateTime&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XGregorianDateTime, ICU4XGregorianDateTimeDeleter> inner;
};


inline diplomat::result<ICU4XGregorianDateTime, ICU4XError> ICU4XGregorianDateTime::try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second) {
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateTime_try_new(year, month, day, hour, minute, second);
  diplomat::result<ICU4XGregorianDateTime, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XGregorianDateTime>(std::move(ICU4XGregorianDateTime(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif

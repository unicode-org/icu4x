#ifndef ICU4XDateTime_HPP
#define ICU4XDateTime_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XDateTime.h"

class ICU4XCalendar;
class ICU4XDateTime;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XDateTime with std::unique_ptr.
 */
struct ICU4XDateTimeDeleter {
  void operator()(capi::ICU4XDateTime* l) const noexcept {
    capi::ICU4XDateTime_destroy(l);
  }
};

/**
 * An ICU4X DateTime object capable of containing a date and time for any calendar.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html) for more information.
 */
class ICU4XDateTime {
 public:

  /**
   * Creates a new [`ICU4XDateTime`] representing the ISO date and time
   * given but in a given calendar
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime) for more information.
   */
  static diplomat::result<ICU4XDateTime, ICU4XError> try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, const ICU4XCalendar& calendar);
  inline const capi::ICU4XDateTime* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDateTime* AsFFIMut() { return this->inner.get(); }
  inline ICU4XDateTime(capi::ICU4XDateTime* i) : inner(i) {}
  ICU4XDateTime() = default;
  ICU4XDateTime(ICU4XDateTime&&) noexcept = default;
  ICU4XDateTime& operator=(ICU4XDateTime&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XDateTime, ICU4XDateTimeDeleter> inner;
};

#include "ICU4XCalendar.hpp"

inline diplomat::result<ICU4XDateTime, ICU4XError> ICU4XDateTime::try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, const ICU4XCalendar& calendar) {
  auto diplomat_result_raw_out_value = capi::ICU4XDateTime_try_new_from_iso_in_calendar(year, month, day, hour, minute, second, calendar.AsFFI());
  diplomat::result<ICU4XDateTime, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XDateTime>(std::move(ICU4XDateTime(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif

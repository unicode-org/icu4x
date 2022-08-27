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
class ICU4XDate;
class ICU4XTime;
class ICU4XIsoDateTime;

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
 * See the [Rust documentation for `DateTime`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html) for more information.
 */
class ICU4XDateTime {
 public:

  /**
   * Creates a new [`ICU4XDateTime`] representing the ISO date and time
   * given but in a given calendar
   * 
   * See the [Rust documentation for `new_iso_datetime`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime) for more information.
   */
  static diplomat::result<ICU4XDateTime, ICU4XError> try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar);

  /**
   * Creates a new [`ICU4XDateTime`] representing the ISO date and time
   * given but in a given calendar
   * 
   * See the [Rust documentation for `new_from_codes`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_from_codes) for more information.
   */
  static diplomat::result<ICU4XDateTime, ICU4XError> try_new_from_codes_in_calendar(const std::string_view era_code, int32_t year, const std::string_view month_code, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar);

  /**
   * Creates a new [`ICU4XDateTime`] from an [`ICU4XDate`] and [`ICU4XTime`] object
   * 
   * See the [Rust documentation for `new`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new) for more information.
   */
  static ICU4XDateTime new_from_date_and_time(const ICU4XDate& date, const ICU4XTime& time);

  /**
   * Gets a copy of the date contained in this object
   * 
   * See the [Rust documentation for `date`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date) for more information.
   */
  ICU4XDate date() const;

  /**
   * Gets the time contained in this object
   * 
   * See the [Rust documentation for `time`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time) for more information.
   */
  ICU4XTime time() const;

  /**
   * Converts this date to ISO
   * 
   * See the [Rust documentation for `to_iso`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_iso) for more information.
   */
  ICU4XIsoDateTime to_iso() const;

  /**
   * Convert this datetime to one in a different calendar
   * 
   * See the [Rust documentation for `to_calendar`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_calendar) for more information.
   */
  ICU4XDateTime to_calendar(const ICU4XCalendar& calendar) const;
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
#include "ICU4XDate.hpp"
#include "ICU4XTime.hpp"
#include "ICU4XIsoDateTime.hpp"

inline diplomat::result<ICU4XDateTime, ICU4XError> ICU4XDateTime::try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar) {
  auto diplomat_result_raw_out_value = capi::ICU4XDateTime_try_new_from_iso_in_calendar(year, month, day, hour, minute, second, nanosecond, calendar.AsFFI());
  diplomat::result<ICU4XDateTime, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XDateTime>(std::move(ICU4XDateTime(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XDateTime, ICU4XError> ICU4XDateTime::try_new_from_codes_in_calendar(const std::string_view era_code, int32_t year, const std::string_view month_code, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar) {
  auto diplomat_result_raw_out_value = capi::ICU4XDateTime_try_new_from_codes_in_calendar(era_code.data(), era_code.size(), year, month_code.data(), month_code.size(), day, hour, minute, second, nanosecond, calendar.AsFFI());
  diplomat::result<ICU4XDateTime, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XDateTime>(std::move(ICU4XDateTime(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XDateTime ICU4XDateTime::new_from_date_and_time(const ICU4XDate& date, const ICU4XTime& time) {
  return ICU4XDateTime(capi::ICU4XDateTime_new_from_date_and_time(date.AsFFI(), time.AsFFI()));
}
inline ICU4XDate ICU4XDateTime::date() const {
  return ICU4XDate(capi::ICU4XDateTime_date(this->inner.get()));
}
inline ICU4XTime ICU4XDateTime::time() const {
  return ICU4XTime(capi::ICU4XDateTime_time(this->inner.get()));
}
inline ICU4XIsoDateTime ICU4XDateTime::to_iso() const {
  return ICU4XIsoDateTime(capi::ICU4XDateTime_to_iso(this->inner.get()));
}
inline ICU4XDateTime ICU4XDateTime::to_calendar(const ICU4XCalendar& calendar) const {
  return ICU4XDateTime(capi::ICU4XDateTime_to_calendar(this->inner.get(), calendar.AsFFI()));
}
#endif

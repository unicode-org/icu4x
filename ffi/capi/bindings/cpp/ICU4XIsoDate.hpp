#ifndef ICU4XIsoDate_HPP
#define ICU4XIsoDate_HPP

#include "ICU4XIsoDate.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendar.hpp"
#include "ICU4XDate.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIsoDate.h"
#include "ICU4XIsoWeekday.hpp"
#include "ICU4XWeekCalculator.hpp"
#include "ICU4XWeekOf.hpp"


inline diplomat::result<std::unique_ptr<ICU4XIsoDate>, ICU4XError> ICU4XIsoDate::create(int32_t year, uint8_t month, uint8_t day) {
  auto result = capi::ICU4XIsoDate_create(year,
    month,
    day);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XIsoDate>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XIsoDate>>(std::unique_ptr<ICU4XIsoDate>(ICU4XIsoDate::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XIsoDate>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XIsoDate> ICU4XIsoDate::create_for_unix_epoch() {
  auto result = capi::ICU4XIsoDate_create_for_unix_epoch();
  return std::unique_ptr<ICU4XIsoDate>(ICU4XIsoDate::FromFFI(result));
}

inline std::unique_ptr<ICU4XDate> ICU4XIsoDate::to_calendar(const ICU4XCalendar& calendar) const {
  auto result = capi::ICU4XIsoDate_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<ICU4XDate>(ICU4XDate::FromFFI(result));
}

inline std::unique_ptr<ICU4XDate> ICU4XIsoDate::to_any() const {
  auto result = capi::ICU4XIsoDate_to_any(this->AsFFI());
  return std::unique_ptr<ICU4XDate>(ICU4XDate::FromFFI(result));
}

inline uint16_t ICU4XIsoDate::day_of_year() const {
  auto result = capi::ICU4XIsoDate_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t ICU4XIsoDate::day_of_month() const {
  auto result = capi::ICU4XIsoDate_day_of_month(this->AsFFI());
  return result;
}

inline ICU4XIsoWeekday ICU4XIsoDate::day_of_week() const {
  auto result = capi::ICU4XIsoDate_day_of_week(this->AsFFI());
  return ICU4XIsoWeekday::FromFFI(result);
}

inline uint32_t ICU4XIsoDate::week_of_month(ICU4XIsoWeekday first_weekday) const {
  auto result = capi::ICU4XIsoDate_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline diplomat::result<ICU4XWeekOf, ICU4XError> ICU4XIsoDate::week_of_year(const ICU4XWeekCalculator& calculator) const {
  auto result = capi::ICU4XIsoDate_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return result.is_ok ? diplomat::result<ICU4XWeekOf, ICU4XError>(diplomat::Ok<ICU4XWeekOf>(ICU4XWeekOf::FromFFI(result.ok))) : diplomat::result<ICU4XWeekOf, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline uint32_t ICU4XIsoDate::month() const {
  auto result = capi::ICU4XIsoDate_month(this->AsFFI());
  return result;
}

inline int32_t ICU4XIsoDate::year() const {
  auto result = capi::ICU4XIsoDate_year(this->AsFFI());
  return result;
}

inline bool ICU4XIsoDate::is_in_leap_year() const {
  auto result = capi::ICU4XIsoDate_is_in_leap_year(this->AsFFI());
  return result;
}

inline uint8_t ICU4XIsoDate::months_in_year() const {
  auto result = capi::ICU4XIsoDate_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t ICU4XIsoDate::days_in_month() const {
  auto result = capi::ICU4XIsoDate_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t ICU4XIsoDate::days_in_year() const {
  auto result = capi::ICU4XIsoDate_days_in_year(this->AsFFI());
  return result;
}

inline const capi::ICU4XIsoDate* ICU4XIsoDate::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XIsoDate*>(this);
}

inline capi::ICU4XIsoDate* ICU4XIsoDate::AsFFI() {
  return reinterpret_cast<capi::ICU4XIsoDate*>(this);
}

inline const ICU4XIsoDate* ICU4XIsoDate::FromFFI(const capi::ICU4XIsoDate* ptr) {
  return reinterpret_cast<const ICU4XIsoDate*>(ptr);
}

inline ICU4XIsoDate* ICU4XIsoDate::FromFFI(capi::ICU4XIsoDate* ptr) {
  return reinterpret_cast<ICU4XIsoDate*>(ptr);
}

inline void ICU4XIsoDate::operator delete(void* ptr) {
  capi::ICU4XIsoDate_destroy(reinterpret_cast<capi::ICU4XIsoDate*>(ptr));
}


#endif // ICU4XIsoDate_HPP

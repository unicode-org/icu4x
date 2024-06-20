#ifndef ICU4XDate_HPP
#define ICU4XDate_HPP

#include "ICU4XDate.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendar.hpp"
#include "ICU4XCalendarError.hpp"
#include "ICU4XDate.h"
#include "ICU4XIsoDate.hpp"
#include "ICU4XIsoWeekday.hpp"
#include "ICU4XWeekCalculator.hpp"
#include "ICU4XWeekOf.hpp"


inline diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError> ICU4XDate::create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, const ICU4XCalendar& calendar) {
  auto result = capi::ICU4XDate_create_from_iso_in_calendar(year,
    month,
    day,
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError>(diplomat::Ok<std::unique_ptr<ICU4XDate>>(std::unique_ptr<ICU4XDate>(ICU4XDate::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError>(diplomat::Err<ICU4XCalendarError>(ICU4XCalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError> ICU4XDate::create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, const ICU4XCalendar& calendar) {
  auto result = capi::ICU4XDate_create_from_codes_in_calendar(era_code.data(),
    era_code.size(),
    year,
    month_code.data(),
    month_code.size(),
    day,
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError>(diplomat::Ok<std::unique_ptr<ICU4XDate>>(std::unique_ptr<ICU4XDate>(ICU4XDate::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError>(diplomat::Err<ICU4XCalendarError>(ICU4XCalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XDate> ICU4XDate::to_calendar(const ICU4XCalendar& calendar) const {
  auto result = capi::ICU4XDate_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<ICU4XDate>(ICU4XDate::FromFFI(result));
}

inline std::unique_ptr<ICU4XIsoDate> ICU4XDate::to_iso() const {
  auto result = capi::ICU4XDate_to_iso(this->AsFFI());
  return std::unique_ptr<ICU4XIsoDate>(ICU4XIsoDate::FromFFI(result));
}

inline uint16_t ICU4XDate::day_of_year() const {
  auto result = capi::ICU4XDate_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t ICU4XDate::day_of_month() const {
  auto result = capi::ICU4XDate_day_of_month(this->AsFFI());
  return result;
}

inline ICU4XIsoWeekday ICU4XDate::day_of_week() const {
  auto result = capi::ICU4XDate_day_of_week(this->AsFFI());
  return ICU4XIsoWeekday::FromFFI(result);
}

inline uint32_t ICU4XDate::week_of_month(ICU4XIsoWeekday first_weekday) const {
  auto result = capi::ICU4XDate_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline ICU4XWeekOf ICU4XDate::week_of_year(const ICU4XWeekCalculator& calculator) const {
  auto result = capi::ICU4XDate_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return ICU4XWeekOf::FromFFI(result);
}

inline uint32_t ICU4XDate::ordinal_month() const {
  auto result = capi::ICU4XDate_ordinal_month(this->AsFFI());
  return result;
}

inline std::string ICU4XDate::month_code() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDate_month_code(this->AsFFI(),
    &write);
  return output;
}

inline int32_t ICU4XDate::year_in_era() const {
  auto result = capi::ICU4XDate_year_in_era(this->AsFFI());
  return result;
}

inline std::string ICU4XDate::era() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDate_era(this->AsFFI(),
    &write);
  return output;
}

inline uint8_t ICU4XDate::months_in_year() const {
  auto result = capi::ICU4XDate_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t ICU4XDate::days_in_month() const {
  auto result = capi::ICU4XDate_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t ICU4XDate::days_in_year() const {
  auto result = capi::ICU4XDate_days_in_year(this->AsFFI());
  return result;
}

inline std::unique_ptr<ICU4XCalendar> ICU4XDate::calendar() const {
  auto result = capi::ICU4XDate_calendar(this->AsFFI());
  return std::unique_ptr<ICU4XCalendar>(ICU4XCalendar::FromFFI(result));
}

inline const capi::ICU4XDate* ICU4XDate::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDate*>(this);
}

inline capi::ICU4XDate* ICU4XDate::AsFFI() {
  return reinterpret_cast<capi::ICU4XDate*>(this);
}

inline const ICU4XDate* ICU4XDate::FromFFI(const capi::ICU4XDate* ptr) {
  return reinterpret_cast<const ICU4XDate*>(ptr);
}

inline ICU4XDate* ICU4XDate::FromFFI(capi::ICU4XDate* ptr) {
  return reinterpret_cast<ICU4XDate*>(ptr);
}

inline void ICU4XDate::operator delete(void* ptr) {
  capi::ICU4XDate_destroy(reinterpret_cast<capi::ICU4XDate*>(ptr));
}


#endif // ICU4XDate_HPP

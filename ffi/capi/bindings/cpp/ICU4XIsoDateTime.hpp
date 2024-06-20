#ifndef ICU4XIsoDateTime_HPP
#define ICU4XIsoDateTime_HPP

#include "ICU4XIsoDateTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendar.hpp"
#include "ICU4XCalendarError.hpp"
#include "ICU4XDateTime.hpp"
#include "ICU4XIsoDate.hpp"
#include "ICU4XIsoWeekday.hpp"
#include "ICU4XTime.hpp"
#include "ICU4XWeekCalculator.hpp"
#include "ICU4XWeekOf.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XIsoDateTime_create_result {union {ICU4XIsoDateTime* ok; ICU4XCalendarError err;}; bool is_ok;};
    struct ICU4XIsoDateTime_create_result ICU4XIsoDateTime_create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);
    
    ICU4XIsoDateTime* ICU4XIsoDateTime_crate_from_date_and_time(const ICU4XIsoDate* date, const ICU4XTime* time);
    
    ICU4XIsoDateTime* ICU4XIsoDateTime_local_unix_epoch();
    
    ICU4XIsoDateTime* ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch(int32_t minutes);
    
    ICU4XIsoDate* ICU4XIsoDateTime_date(const ICU4XIsoDateTime* self);
    
    ICU4XTime* ICU4XIsoDateTime_time(const ICU4XIsoDateTime* self);
    
    ICU4XDateTime* ICU4XIsoDateTime_to_any(const ICU4XIsoDateTime* self);
    
    int32_t ICU4XIsoDateTime_minutes_since_local_unix_epoch(const ICU4XIsoDateTime* self);
    
    ICU4XDateTime* ICU4XIsoDateTime_to_calendar(const ICU4XIsoDateTime* self, const ICU4XCalendar* calendar);
    
    uint8_t ICU4XIsoDateTime_hour(const ICU4XIsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_minute(const ICU4XIsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_second(const ICU4XIsoDateTime* self);
    
    uint32_t ICU4XIsoDateTime_nanosecond(const ICU4XIsoDateTime* self);
    
    uint16_t ICU4XIsoDateTime_day_of_year(const ICU4XIsoDateTime* self);
    
    uint32_t ICU4XIsoDateTime_day_of_month(const ICU4XIsoDateTime* self);
    
    ICU4XIsoWeekday ICU4XIsoDateTime_day_of_week(const ICU4XIsoDateTime* self);
    
    uint32_t ICU4XIsoDateTime_week_of_month(const ICU4XIsoDateTime* self, ICU4XIsoWeekday first_weekday);
    
    ICU4XWeekOf ICU4XIsoDateTime_week_of_year(const ICU4XIsoDateTime* self, const ICU4XWeekCalculator* calculator);
    
    uint32_t ICU4XIsoDateTime_month(const ICU4XIsoDateTime* self);
    
    int32_t ICU4XIsoDateTime_year(const ICU4XIsoDateTime* self);
    
    bool ICU4XIsoDateTime_is_in_leap_year(const ICU4XIsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_months_in_year(const ICU4XIsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_days_in_month(const ICU4XIsoDateTime* self);
    
    uint16_t ICU4XIsoDateTime_days_in_year(const ICU4XIsoDateTime* self);
    
    
    void ICU4XIsoDateTime_destroy(ICU4XIsoDateTime* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XIsoDateTime>, ICU4XCalendarError> ICU4XIsoDateTime::create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto result = capi::ICU4XIsoDateTime_create(year,
    month,
    day,
    hour,
    minute,
    second,
    nanosecond);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XIsoDateTime>, ICU4XCalendarError>(diplomat::Ok<std::unique_ptr<ICU4XIsoDateTime>>(std::unique_ptr<ICU4XIsoDateTime>(ICU4XIsoDateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XIsoDateTime>, ICU4XCalendarError>(diplomat::Err<ICU4XCalendarError>(ICU4XCalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XIsoDateTime> ICU4XIsoDateTime::crate_from_date_and_time(const ICU4XIsoDate& date, const ICU4XTime& time) {
  auto result = capi::ICU4XIsoDateTime_crate_from_date_and_time(date.AsFFI(),
    time.AsFFI());
  return std::unique_ptr<ICU4XIsoDateTime>(ICU4XIsoDateTime::FromFFI(result));
}

inline std::unique_ptr<ICU4XIsoDateTime> ICU4XIsoDateTime::local_unix_epoch() {
  auto result = capi::ICU4XIsoDateTime_local_unix_epoch();
  return std::unique_ptr<ICU4XIsoDateTime>(ICU4XIsoDateTime::FromFFI(result));
}

inline std::unique_ptr<ICU4XIsoDateTime> ICU4XIsoDateTime::create_from_minutes_since_local_unix_epoch(int32_t minutes) {
  auto result = capi::ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch(minutes);
  return std::unique_ptr<ICU4XIsoDateTime>(ICU4XIsoDateTime::FromFFI(result));
}

inline std::unique_ptr<ICU4XIsoDate> ICU4XIsoDateTime::date() const {
  auto result = capi::ICU4XIsoDateTime_date(this->AsFFI());
  return std::unique_ptr<ICU4XIsoDate>(ICU4XIsoDate::FromFFI(result));
}

inline std::unique_ptr<ICU4XTime> ICU4XIsoDateTime::time() const {
  auto result = capi::ICU4XIsoDateTime_time(this->AsFFI());
  return std::unique_ptr<ICU4XTime>(ICU4XTime::FromFFI(result));
}

inline std::unique_ptr<ICU4XDateTime> ICU4XIsoDateTime::to_any() const {
  auto result = capi::ICU4XIsoDateTime_to_any(this->AsFFI());
  return std::unique_ptr<ICU4XDateTime>(ICU4XDateTime::FromFFI(result));
}

inline int32_t ICU4XIsoDateTime::minutes_since_local_unix_epoch() const {
  auto result = capi::ICU4XIsoDateTime_minutes_since_local_unix_epoch(this->AsFFI());
  return result;
}

inline std::unique_ptr<ICU4XDateTime> ICU4XIsoDateTime::to_calendar(const ICU4XCalendar& calendar) const {
  auto result = capi::ICU4XIsoDateTime_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<ICU4XDateTime>(ICU4XDateTime::FromFFI(result));
}

inline uint8_t ICU4XIsoDateTime::hour() const {
  auto result = capi::ICU4XIsoDateTime_hour(this->AsFFI());
  return result;
}

inline uint8_t ICU4XIsoDateTime::minute() const {
  auto result = capi::ICU4XIsoDateTime_minute(this->AsFFI());
  return result;
}

inline uint8_t ICU4XIsoDateTime::second() const {
  auto result = capi::ICU4XIsoDateTime_second(this->AsFFI());
  return result;
}

inline uint32_t ICU4XIsoDateTime::nanosecond() const {
  auto result = capi::ICU4XIsoDateTime_nanosecond(this->AsFFI());
  return result;
}

inline uint16_t ICU4XIsoDateTime::day_of_year() const {
  auto result = capi::ICU4XIsoDateTime_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t ICU4XIsoDateTime::day_of_month() const {
  auto result = capi::ICU4XIsoDateTime_day_of_month(this->AsFFI());
  return result;
}

inline ICU4XIsoWeekday ICU4XIsoDateTime::day_of_week() const {
  auto result = capi::ICU4XIsoDateTime_day_of_week(this->AsFFI());
  return ICU4XIsoWeekday::FromFFI(result);
}

inline uint32_t ICU4XIsoDateTime::week_of_month(ICU4XIsoWeekday first_weekday) const {
  auto result = capi::ICU4XIsoDateTime_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline ICU4XWeekOf ICU4XIsoDateTime::week_of_year(const ICU4XWeekCalculator& calculator) const {
  auto result = capi::ICU4XIsoDateTime_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return ICU4XWeekOf::FromFFI(result);
}

inline uint32_t ICU4XIsoDateTime::month() const {
  auto result = capi::ICU4XIsoDateTime_month(this->AsFFI());
  return result;
}

inline int32_t ICU4XIsoDateTime::year() const {
  auto result = capi::ICU4XIsoDateTime_year(this->AsFFI());
  return result;
}

inline bool ICU4XIsoDateTime::is_in_leap_year() const {
  auto result = capi::ICU4XIsoDateTime_is_in_leap_year(this->AsFFI());
  return result;
}

inline uint8_t ICU4XIsoDateTime::months_in_year() const {
  auto result = capi::ICU4XIsoDateTime_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t ICU4XIsoDateTime::days_in_month() const {
  auto result = capi::ICU4XIsoDateTime_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t ICU4XIsoDateTime::days_in_year() const {
  auto result = capi::ICU4XIsoDateTime_days_in_year(this->AsFFI());
  return result;
}

inline const capi::ICU4XIsoDateTime* ICU4XIsoDateTime::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XIsoDateTime*>(this);
}

inline capi::ICU4XIsoDateTime* ICU4XIsoDateTime::AsFFI() {
  return reinterpret_cast<capi::ICU4XIsoDateTime*>(this);
}

inline const ICU4XIsoDateTime* ICU4XIsoDateTime::FromFFI(const capi::ICU4XIsoDateTime* ptr) {
  return reinterpret_cast<const ICU4XIsoDateTime*>(ptr);
}

inline ICU4XIsoDateTime* ICU4XIsoDateTime::FromFFI(capi::ICU4XIsoDateTime* ptr) {
  return reinterpret_cast<ICU4XIsoDateTime*>(ptr);
}

inline void ICU4XIsoDateTime::operator delete(void* ptr) {
  capi::ICU4XIsoDateTime_destroy(reinterpret_cast<capi::ICU4XIsoDateTime*>(ptr));
}


#endif // ICU4XIsoDateTime_HPP

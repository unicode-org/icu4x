#ifndef IsoDateTime_HPP
#define IsoDateTime_HPP

#include "IsoDateTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Calendar.hpp"
#include "CalendarError.hpp"
#include "DateTime.hpp"
#include "IsoDate.hpp"
#include "IsoWeekday.hpp"
#include "Time.hpp"
#include "WeekCalculator.hpp"
#include "WeekOf.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XIsoDateTime_create_result {union {IsoDateTime* ok; CalendarError err;}; bool is_ok;} ICU4XIsoDateTime_create_result;
    ICU4XIsoDateTime_create_result ICU4XIsoDateTime_create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);
    
    IsoDateTime* ICU4XIsoDateTime_crate_from_date_and_time(const IsoDate* date, const Time* time);
    
    IsoDateTime* ICU4XIsoDateTime_local_unix_epoch();
    
    IsoDateTime* ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch(int32_t minutes);
    
    IsoDate* ICU4XIsoDateTime_date(const IsoDateTime* self);
    
    Time* ICU4XIsoDateTime_time(const IsoDateTime* self);
    
    DateTime* ICU4XIsoDateTime_to_any(const IsoDateTime* self);
    
    int32_t ICU4XIsoDateTime_minutes_since_local_unix_epoch(const IsoDateTime* self);
    
    DateTime* ICU4XIsoDateTime_to_calendar(const IsoDateTime* self, const Calendar* calendar);
    
    uint8_t ICU4XIsoDateTime_hour(const IsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_minute(const IsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_second(const IsoDateTime* self);
    
    uint32_t ICU4XIsoDateTime_nanosecond(const IsoDateTime* self);
    
    uint16_t ICU4XIsoDateTime_day_of_year(const IsoDateTime* self);
    
    uint32_t ICU4XIsoDateTime_day_of_month(const IsoDateTime* self);
    
    IsoWeekday ICU4XIsoDateTime_day_of_week(const IsoDateTime* self);
    
    uint32_t ICU4XIsoDateTime_week_of_month(const IsoDateTime* self, IsoWeekday first_weekday);
    
    WeekOf ICU4XIsoDateTime_week_of_year(const IsoDateTime* self, const WeekCalculator* calculator);
    
    uint32_t ICU4XIsoDateTime_month(const IsoDateTime* self);
    
    int32_t ICU4XIsoDateTime_year(const IsoDateTime* self);
    
    bool ICU4XIsoDateTime_is_in_leap_year(const IsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_months_in_year(const IsoDateTime* self);
    
    uint8_t ICU4XIsoDateTime_days_in_month(const IsoDateTime* self);
    
    uint16_t ICU4XIsoDateTime_days_in_year(const IsoDateTime* self);
    
    
    void ICU4XIsoDateTime_destroy(IsoDateTime* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<IsoDateTime>, CalendarError> IsoDateTime::create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto result = capi::ICU4XIsoDateTime_create(year,
    month,
    day,
    hour,
    minute,
    second,
    nanosecond);
  return result.is_ok ? diplomat::result<std::unique_ptr<IsoDateTime>, CalendarError>(diplomat::Ok<std::unique_ptr<IsoDateTime>>(std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<IsoDateTime>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<IsoDateTime> IsoDateTime::crate_from_date_and_time(const IsoDate& date, const Time& time) {
  auto result = capi::ICU4XIsoDateTime_crate_from_date_and_time(date.AsFFI(),
    time.AsFFI());
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<IsoDateTime> IsoDateTime::local_unix_epoch() {
  auto result = capi::ICU4XIsoDateTime_local_unix_epoch();
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<IsoDateTime> IsoDateTime::create_from_minutes_since_local_unix_epoch(int32_t minutes) {
  auto result = capi::ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch(minutes);
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<IsoDate> IsoDateTime::date() const {
  auto result = capi::ICU4XIsoDateTime_date(this->AsFFI());
  return std::unique_ptr<IsoDate>(IsoDate::FromFFI(result));
}

inline std::unique_ptr<Time> IsoDateTime::time() const {
  auto result = capi::ICU4XIsoDateTime_time(this->AsFFI());
  return std::unique_ptr<Time>(Time::FromFFI(result));
}

inline std::unique_ptr<DateTime> IsoDateTime::to_any() const {
  auto result = capi::ICU4XIsoDateTime_to_any(this->AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline int32_t IsoDateTime::minutes_since_local_unix_epoch() const {
  auto result = capi::ICU4XIsoDateTime_minutes_since_local_unix_epoch(this->AsFFI());
  return result;
}

inline std::unique_ptr<DateTime> IsoDateTime::to_calendar(const Calendar& calendar) const {
  auto result = capi::ICU4XIsoDateTime_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline uint8_t IsoDateTime::hour() const {
  auto result = capi::ICU4XIsoDateTime_hour(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::minute() const {
  auto result = capi::ICU4XIsoDateTime_minute(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::second() const {
  auto result = capi::ICU4XIsoDateTime_second(this->AsFFI());
  return result;
}

inline uint32_t IsoDateTime::nanosecond() const {
  auto result = capi::ICU4XIsoDateTime_nanosecond(this->AsFFI());
  return result;
}

inline uint16_t IsoDateTime::day_of_year() const {
  auto result = capi::ICU4XIsoDateTime_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t IsoDateTime::day_of_month() const {
  auto result = capi::ICU4XIsoDateTime_day_of_month(this->AsFFI());
  return result;
}

inline IsoWeekday IsoDateTime::day_of_week() const {
  auto result = capi::ICU4XIsoDateTime_day_of_week(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t IsoDateTime::week_of_month(IsoWeekday first_weekday) const {
  auto result = capi::ICU4XIsoDateTime_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf IsoDateTime::week_of_year(const WeekCalculator& calculator) const {
  auto result = capi::ICU4XIsoDateTime_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t IsoDateTime::month() const {
  auto result = capi::ICU4XIsoDateTime_month(this->AsFFI());
  return result;
}

inline int32_t IsoDateTime::year() const {
  auto result = capi::ICU4XIsoDateTime_year(this->AsFFI());
  return result;
}

inline bool IsoDateTime::is_in_leap_year() const {
  auto result = capi::ICU4XIsoDateTime_is_in_leap_year(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::months_in_year() const {
  auto result = capi::ICU4XIsoDateTime_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::days_in_month() const {
  auto result = capi::ICU4XIsoDateTime_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t IsoDateTime::days_in_year() const {
  auto result = capi::ICU4XIsoDateTime_days_in_year(this->AsFFI());
  return result;
}

inline const capi::IsoDateTime* IsoDateTime::AsFFI() const {
  return reinterpret_cast<const capi::IsoDateTime*>(this);
}

inline capi::IsoDateTime* IsoDateTime::AsFFI() {
  return reinterpret_cast<capi::IsoDateTime*>(this);
}

inline const IsoDateTime* IsoDateTime::FromFFI(const capi::IsoDateTime* ptr) {
  return reinterpret_cast<const IsoDateTime*>(ptr);
}

inline IsoDateTime* IsoDateTime::FromFFI(capi::IsoDateTime* ptr) {
  return reinterpret_cast<IsoDateTime*>(ptr);
}

inline void IsoDateTime::operator delete(void* ptr) {
  capi::ICU4XIsoDateTime_destroy(reinterpret_cast<capi::IsoDateTime*>(ptr));
}


#endif // IsoDateTime_HPP

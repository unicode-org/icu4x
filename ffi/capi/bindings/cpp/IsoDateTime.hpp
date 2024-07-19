#ifndef IsoDateTime_HPP
#define IsoDateTime_HPP

#include "IsoDateTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "Calendar.hpp"
#include "CalendarError.hpp"
#include "CalendarParseError.hpp"
#include "DateTime.hpp"
#include "IsoDate.hpp"
#include "IsoWeekday.hpp"
#include "Time.hpp"
#include "WeekCalculator.hpp"
#include "WeekOf.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_IsoDateTime_create_mv1_result {union {diplomat::capi::IsoDateTime* ok; diplomat::capi::CalendarError err;}; bool is_ok;} icu4x_IsoDateTime_create_mv1_result;
    icu4x_IsoDateTime_create_mv1_result icu4x_IsoDateTime_create_mv1(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);
    
    diplomat::capi::IsoDateTime* icu4x_IsoDateTime_from_date_and_time_mv1(const diplomat::capi::IsoDate* date, const diplomat::capi::Time* time);
    
<<<<<<< HEAD
    typedef struct icu4x_IsoDateTime_from_string_mv1_result {union {diplomat::capi::IsoDateTime* ok; diplomat::capi::CalendarParseError err;}; bool is_ok;} icu4x_IsoDateTime_from_string_mv1_result;
    icu4x_IsoDateTime_from_string_mv1_result icu4x_IsoDateTime_from_string_mv1(const char* v_data, size_t v_len);
    
    diplomat::capi::IsoDateTime* icu4x_IsoDateTime_local_unix_epoch_mv1();
=======
    diplomat::capi::IsoDateTime* icu4x_IsoDateTime_local_unix_epoch_mv1(void);
>>>>>>> upstream/main
    
    diplomat::capi::IsoDateTime* icu4x_IsoDateTime_from_minutes_since_local_unix_epoch_mv1(int32_t minutes);
    
    diplomat::capi::IsoDate* icu4x_IsoDateTime_date_mv1(const diplomat::capi::IsoDateTime* self);
    
    diplomat::capi::Time* icu4x_IsoDateTime_time_mv1(const diplomat::capi::IsoDateTime* self);
    
    diplomat::capi::DateTime* icu4x_IsoDateTime_to_any_mv1(const diplomat::capi::IsoDateTime* self);
    
    int32_t icu4x_IsoDateTime_minutes_since_local_unix_epoch_mv1(const diplomat::capi::IsoDateTime* self);
    
    diplomat::capi::DateTime* icu4x_IsoDateTime_to_calendar_mv1(const diplomat::capi::IsoDateTime* self, const diplomat::capi::Calendar* calendar);
    
    uint8_t icu4x_IsoDateTime_hour_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint8_t icu4x_IsoDateTime_minute_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint8_t icu4x_IsoDateTime_second_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint32_t icu4x_IsoDateTime_nanosecond_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint16_t icu4x_IsoDateTime_day_of_year_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint32_t icu4x_IsoDateTime_day_of_month_mv1(const diplomat::capi::IsoDateTime* self);
    
    diplomat::capi::IsoWeekday icu4x_IsoDateTime_day_of_week_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint32_t icu4x_IsoDateTime_week_of_month_mv1(const diplomat::capi::IsoDateTime* self, diplomat::capi::IsoWeekday first_weekday);
    
    diplomat::capi::WeekOf icu4x_IsoDateTime_week_of_year_mv1(const diplomat::capi::IsoDateTime* self, const diplomat::capi::WeekCalculator* calculator);
    
    uint32_t icu4x_IsoDateTime_month_mv1(const diplomat::capi::IsoDateTime* self);
    
    int32_t icu4x_IsoDateTime_year_mv1(const diplomat::capi::IsoDateTime* self);
    
    bool icu4x_IsoDateTime_is_in_leap_year_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint8_t icu4x_IsoDateTime_months_in_year_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint8_t icu4x_IsoDateTime_days_in_month_mv1(const diplomat::capi::IsoDateTime* self);
    
    uint16_t icu4x_IsoDateTime_days_in_year_mv1(const diplomat::capi::IsoDateTime* self);
    
    
    void icu4x_IsoDateTime_destroy_mv1(IsoDateTime* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<IsoDateTime>, CalendarError> IsoDateTime::create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto result = diplomat::capi::icu4x_IsoDateTime_create_mv1(year,
    month,
    day,
    hour,
    minute,
    second,
    nanosecond);
  return result.is_ok ? diplomat::result<std::unique_ptr<IsoDateTime>, CalendarError>(diplomat::Ok<std::unique_ptr<IsoDateTime>>(std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<IsoDateTime>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<IsoDateTime> IsoDateTime::from_date_and_time(const IsoDate& date, const Time& time) {
  auto result = diplomat::capi::icu4x_IsoDateTime_from_date_and_time_mv1(date.AsFFI(),
    time.AsFFI());
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<IsoDateTime>, CalendarParseError> IsoDateTime::from_string(std::string_view v) {
  auto result = diplomat::capi::icu4x_IsoDateTime_from_string_mv1(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<IsoDateTime>, CalendarParseError>(diplomat::Ok<std::unique_ptr<IsoDateTime>>(std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<IsoDateTime>, CalendarParseError>(diplomat::Err<CalendarParseError>(CalendarParseError::FromFFI(result.err)));
}

inline std::unique_ptr<IsoDateTime> IsoDateTime::local_unix_epoch() {
  auto result = diplomat::capi::icu4x_IsoDateTime_local_unix_epoch_mv1();
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<IsoDateTime> IsoDateTime::from_minutes_since_local_unix_epoch(int32_t minutes) {
  auto result = diplomat::capi::icu4x_IsoDateTime_from_minutes_since_local_unix_epoch_mv1(minutes);
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<IsoDate> IsoDateTime::date() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_date_mv1(this->AsFFI());
  return std::unique_ptr<IsoDate>(IsoDate::FromFFI(result));
}

inline std::unique_ptr<Time> IsoDateTime::time() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_time_mv1(this->AsFFI());
  return std::unique_ptr<Time>(Time::FromFFI(result));
}

inline std::unique_ptr<DateTime> IsoDateTime::to_any() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_to_any_mv1(this->AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline int32_t IsoDateTime::minutes_since_local_unix_epoch() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_minutes_since_local_unix_epoch_mv1(this->AsFFI());
  return result;
}

inline std::unique_ptr<DateTime> IsoDateTime::to_calendar(const Calendar& calendar) const {
  auto result = diplomat::capi::icu4x_IsoDateTime_to_calendar_mv1(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline uint8_t IsoDateTime::hour() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_hour_mv1(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::minute() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_minute_mv1(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::second() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_second_mv1(this->AsFFI());
  return result;
}

inline uint32_t IsoDateTime::nanosecond() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_nanosecond_mv1(this->AsFFI());
  return result;
}

inline uint16_t IsoDateTime::day_of_year() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_day_of_year_mv1(this->AsFFI());
  return result;
}

inline uint32_t IsoDateTime::day_of_month() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_day_of_month_mv1(this->AsFFI());
  return result;
}

inline IsoWeekday IsoDateTime::day_of_week() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_day_of_week_mv1(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t IsoDateTime::week_of_month(IsoWeekday first_weekday) const {
  auto result = diplomat::capi::icu4x_IsoDateTime_week_of_month_mv1(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf IsoDateTime::week_of_year(const WeekCalculator& calculator) const {
  auto result = diplomat::capi::icu4x_IsoDateTime_week_of_year_mv1(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t IsoDateTime::month() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_month_mv1(this->AsFFI());
  return result;
}

inline int32_t IsoDateTime::year() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_year_mv1(this->AsFFI());
  return result;
}

inline bool IsoDateTime::is_in_leap_year() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_is_in_leap_year_mv1(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::months_in_year() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_months_in_year_mv1(this->AsFFI());
  return result;
}

inline uint8_t IsoDateTime::days_in_month() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_days_in_month_mv1(this->AsFFI());
  return result;
}

inline uint16_t IsoDateTime::days_in_year() const {
  auto result = diplomat::capi::icu4x_IsoDateTime_days_in_year_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::IsoDateTime* IsoDateTime::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::IsoDateTime*>(this);
}

inline diplomat::capi::IsoDateTime* IsoDateTime::AsFFI() {
  return reinterpret_cast<diplomat::capi::IsoDateTime*>(this);
}

inline const IsoDateTime* IsoDateTime::FromFFI(const diplomat::capi::IsoDateTime* ptr) {
  return reinterpret_cast<const IsoDateTime*>(ptr);
}

inline IsoDateTime* IsoDateTime::FromFFI(diplomat::capi::IsoDateTime* ptr) {
  return reinterpret_cast<IsoDateTime*>(ptr);
}

inline void IsoDateTime::operator delete(void* ptr) {
  diplomat::capi::icu4x_IsoDateTime_destroy_mv1(reinterpret_cast<diplomat::capi::IsoDateTime*>(ptr));
}


#endif // IsoDateTime_HPP

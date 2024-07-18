#ifndef DateTime_HPP
#define DateTime_HPP

#include "DateTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Calendar.hpp"
#include "CalendarError.hpp"
#include "Date.hpp"
#include "FromIxdtfError.hpp"
#include "IsoDateTime.hpp"
#include "IsoWeekday.hpp"
#include "Time.hpp"
#include "WeekCalculator.hpp"
#include "WeekOf.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_DateTime_create_from_iso_in_calendar_mv1_result {union {diplomat::capi::DateTime* ok; diplomat::capi::CalendarError err;}; bool is_ok;} icu4x_DateTime_create_from_iso_in_calendar_mv1_result;
    icu4x_DateTime_create_from_iso_in_calendar_mv1_result icu4x_DateTime_create_from_iso_in_calendar_mv1(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const diplomat::capi::Calendar* calendar);
    
    typedef struct icu4x_DateTime_create_from_codes_in_calendar_mv1_result {union {diplomat::capi::DateTime* ok; diplomat::capi::CalendarError err;}; bool is_ok;} icu4x_DateTime_create_from_codes_in_calendar_mv1_result;
    icu4x_DateTime_create_from_codes_in_calendar_mv1_result icu4x_DateTime_create_from_codes_in_calendar_mv1(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const diplomat::capi::Calendar* calendar);
    
    diplomat::capi::DateTime* icu4x_DateTime_create_from_date_and_time_mv1(const diplomat::capi::Date* date, const diplomat::capi::Time* time);
    
    typedef struct icu4x_DateTime_create_from_string_mv1_result {union {diplomat::capi::DateTime* ok; diplomat::capi::FromIxdtfError err;}; bool is_ok;} icu4x_DateTime_create_from_string_mv1_result;
    icu4x_DateTime_create_from_string_mv1_result icu4x_DateTime_create_from_string_mv1(const char* v_data, size_t v_len);
    
    diplomat::capi::Date* icu4x_DateTime_date_mv1(const diplomat::capi::DateTime* self);
    
    diplomat::capi::Time* icu4x_DateTime_time_mv1(const diplomat::capi::DateTime* self);
    
    diplomat::capi::IsoDateTime* icu4x_DateTime_to_iso_mv1(const diplomat::capi::DateTime* self);
    
    diplomat::capi::DateTime* icu4x_DateTime_to_calendar_mv1(const diplomat::capi::DateTime* self, const diplomat::capi::Calendar* calendar);
    
    uint8_t icu4x_DateTime_hour_mv1(const diplomat::capi::DateTime* self);
    
    uint8_t icu4x_DateTime_minute_mv1(const diplomat::capi::DateTime* self);
    
    uint8_t icu4x_DateTime_second_mv1(const diplomat::capi::DateTime* self);
    
    uint32_t icu4x_DateTime_nanosecond_mv1(const diplomat::capi::DateTime* self);
    
    uint16_t icu4x_DateTime_day_of_year_mv1(const diplomat::capi::DateTime* self);
    
    uint32_t icu4x_DateTime_day_of_month_mv1(const diplomat::capi::DateTime* self);
    
    diplomat::capi::IsoWeekday icu4x_DateTime_day_of_week_mv1(const diplomat::capi::DateTime* self);
    
    uint32_t icu4x_DateTime_week_of_month_mv1(const diplomat::capi::DateTime* self, diplomat::capi::IsoWeekday first_weekday);
    
    diplomat::capi::WeekOf icu4x_DateTime_week_of_year_mv1(const diplomat::capi::DateTime* self, const diplomat::capi::WeekCalculator* calculator);
    
    uint32_t icu4x_DateTime_ordinal_month_mv1(const diplomat::capi::DateTime* self);
    
    void icu4x_DateTime_month_code_mv1(const diplomat::capi::DateTime* self, diplomat::capi::DiplomatWrite* write);
    
    int32_t icu4x_DateTime_year_in_era_mv1(const diplomat::capi::DateTime* self);
    
    void icu4x_DateTime_era_mv1(const diplomat::capi::DateTime* self, diplomat::capi::DiplomatWrite* write);
    
    uint8_t icu4x_DateTime_months_in_year_mv1(const diplomat::capi::DateTime* self);
    
    uint8_t icu4x_DateTime_days_in_month_mv1(const diplomat::capi::DateTime* self);
    
    uint16_t icu4x_DateTime_days_in_year_mv1(const diplomat::capi::DateTime* self);
    
    diplomat::capi::Calendar* icu4x_DateTime_calendar_mv1(const diplomat::capi::DateTime* self);
    
    
    void icu4x_DateTime_destroy_mv1(DateTime* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<DateTime>, CalendarError> DateTime::create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar& calendar) {
  auto result = diplomat::capi::icu4x_DateTime_create_from_iso_in_calendar_mv1(year,
    month,
    day,
    hour,
    minute,
    second,
    nanosecond,
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DateTime>, CalendarError>(diplomat::Ok<std::unique_ptr<DateTime>>(std::unique_ptr<DateTime>(DateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DateTime>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<DateTime>, CalendarError> DateTime::create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar& calendar) {
  auto result = diplomat::capi::icu4x_DateTime_create_from_codes_in_calendar_mv1(era_code.data(),
    era_code.size(),
    year,
    month_code.data(),
    month_code.size(),
    day,
    hour,
    minute,
    second,
    nanosecond,
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DateTime>, CalendarError>(diplomat::Ok<std::unique_ptr<DateTime>>(std::unique_ptr<DateTime>(DateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DateTime>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<DateTime> DateTime::create_from_date_and_time(const Date& date, const Time& time) {
  auto result = diplomat::capi::icu4x_DateTime_create_from_date_and_time_mv1(date.AsFFI(),
    time.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<DateTime>, FromIxdtfError> DateTime::create_from_string(std::string_view v) {
  auto result = diplomat::capi::icu4x_DateTime_create_from_string_mv1(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<DateTime>, FromIxdtfError>(diplomat::Ok<std::unique_ptr<DateTime>>(std::unique_ptr<DateTime>(DateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DateTime>, FromIxdtfError>(diplomat::Err<FromIxdtfError>(FromIxdtfError::FromFFI(result.err)));
}

inline std::unique_ptr<Date> DateTime::date() const {
  auto result = diplomat::capi::icu4x_DateTime_date_mv1(this->AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline std::unique_ptr<Time> DateTime::time() const {
  auto result = diplomat::capi::icu4x_DateTime_time_mv1(this->AsFFI());
  return std::unique_ptr<Time>(Time::FromFFI(result));
}

inline std::unique_ptr<IsoDateTime> DateTime::to_iso() const {
  auto result = diplomat::capi::icu4x_DateTime_to_iso_mv1(this->AsFFI());
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<DateTime> DateTime::to_calendar(const Calendar& calendar) const {
  auto result = diplomat::capi::icu4x_DateTime_to_calendar_mv1(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline uint8_t DateTime::hour() const {
  auto result = diplomat::capi::icu4x_DateTime_hour_mv1(this->AsFFI());
  return result;
}

inline uint8_t DateTime::minute() const {
  auto result = diplomat::capi::icu4x_DateTime_minute_mv1(this->AsFFI());
  return result;
}

inline uint8_t DateTime::second() const {
  auto result = diplomat::capi::icu4x_DateTime_second_mv1(this->AsFFI());
  return result;
}

inline uint32_t DateTime::nanosecond() const {
  auto result = diplomat::capi::icu4x_DateTime_nanosecond_mv1(this->AsFFI());
  return result;
}

inline uint16_t DateTime::day_of_year() const {
  auto result = diplomat::capi::icu4x_DateTime_day_of_year_mv1(this->AsFFI());
  return result;
}

inline uint32_t DateTime::day_of_month() const {
  auto result = diplomat::capi::icu4x_DateTime_day_of_month_mv1(this->AsFFI());
  return result;
}

inline IsoWeekday DateTime::day_of_week() const {
  auto result = diplomat::capi::icu4x_DateTime_day_of_week_mv1(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t DateTime::week_of_month(IsoWeekday first_weekday) const {
  auto result = diplomat::capi::icu4x_DateTime_week_of_month_mv1(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf DateTime::week_of_year(const WeekCalculator& calculator) const {
  auto result = diplomat::capi::icu4x_DateTime_week_of_year_mv1(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t DateTime::ordinal_month() const {
  auto result = diplomat::capi::icu4x_DateTime_ordinal_month_mv1(this->AsFFI());
  return result;
}

inline std::string DateTime::month_code() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_DateTime_month_code_mv1(this->AsFFI(),
    &write);
  return output;
}

inline int32_t DateTime::year_in_era() const {
  auto result = diplomat::capi::icu4x_DateTime_year_in_era_mv1(this->AsFFI());
  return result;
}

inline std::string DateTime::era() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_DateTime_era_mv1(this->AsFFI(),
    &write);
  return output;
}

inline uint8_t DateTime::months_in_year() const {
  auto result = diplomat::capi::icu4x_DateTime_months_in_year_mv1(this->AsFFI());
  return result;
}

inline uint8_t DateTime::days_in_month() const {
  auto result = diplomat::capi::icu4x_DateTime_days_in_month_mv1(this->AsFFI());
  return result;
}

inline uint16_t DateTime::days_in_year() const {
  auto result = diplomat::capi::icu4x_DateTime_days_in_year_mv1(this->AsFFI());
  return result;
}

inline std::unique_ptr<Calendar> DateTime::calendar() const {
  auto result = diplomat::capi::icu4x_DateTime_calendar_mv1(this->AsFFI());
  return std::unique_ptr<Calendar>(Calendar::FromFFI(result));
}

inline const diplomat::capi::DateTime* DateTime::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::DateTime*>(this);
}

inline diplomat::capi::DateTime* DateTime::AsFFI() {
  return reinterpret_cast<diplomat::capi::DateTime*>(this);
}

inline const DateTime* DateTime::FromFFI(const diplomat::capi::DateTime* ptr) {
  return reinterpret_cast<const DateTime*>(ptr);
}

inline DateTime* DateTime::FromFFI(diplomat::capi::DateTime* ptr) {
  return reinterpret_cast<DateTime*>(ptr);
}

inline void DateTime::operator delete(void* ptr) {
  diplomat::capi::icu4x_DateTime_destroy_mv1(reinterpret_cast<diplomat::capi::DateTime*>(ptr));
}


#endif // DateTime_HPP

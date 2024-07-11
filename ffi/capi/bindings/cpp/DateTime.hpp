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
#include "IsoDateTime.hpp"
#include "IsoWeekday.hpp"
#include "Time.hpp"
#include "WeekCalculator.hpp"
#include "WeekOf.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XDateTime_create_from_iso_in_calendar_result {union {DateTime* ok; CalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_iso_in_calendar_result;
    ICU4XDateTime_create_from_iso_in_calendar_result ICU4XDateTime_create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar* calendar);
    
    typedef struct ICU4XDateTime_create_from_codes_in_calendar_result {union {DateTime* ok; CalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_codes_in_calendar_result;
    ICU4XDateTime_create_from_codes_in_calendar_result ICU4XDateTime_create_from_codes_in_calendar(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar* calendar);
    
    DateTime* ICU4XDateTime_create_from_date_and_time(const Date* date, const Time* time);
    
    Date* ICU4XDateTime_date(const DateTime* self);
    
    Time* ICU4XDateTime_time(const DateTime* self);
    
    IsoDateTime* ICU4XDateTime_to_iso(const DateTime* self);
    
    DateTime* ICU4XDateTime_to_calendar(const DateTime* self, const Calendar* calendar);
    
    uint8_t ICU4XDateTime_hour(const DateTime* self);
    
    uint8_t ICU4XDateTime_minute(const DateTime* self);
    
    uint8_t ICU4XDateTime_second(const DateTime* self);
    
    uint32_t ICU4XDateTime_nanosecond(const DateTime* self);
    
    uint16_t ICU4XDateTime_day_of_year(const DateTime* self);
    
    uint32_t ICU4XDateTime_day_of_month(const DateTime* self);
    
    IsoWeekday ICU4XDateTime_day_of_week(const DateTime* self);
    
    uint32_t ICU4XDateTime_week_of_month(const DateTime* self, IsoWeekday first_weekday);
    
    WeekOf ICU4XDateTime_week_of_year(const DateTime* self, const WeekCalculator* calculator);
    
    uint32_t ICU4XDateTime_ordinal_month(const DateTime* self);
    
    void ICU4XDateTime_month_code(const DateTime* self, DiplomatWrite* write);
    
    int32_t ICU4XDateTime_year_in_era(const DateTime* self);
    
    void ICU4XDateTime_era(const DateTime* self, DiplomatWrite* write);
    
    uint8_t ICU4XDateTime_months_in_year(const DateTime* self);
    
    uint8_t ICU4XDateTime_days_in_month(const DateTime* self);
    
    uint16_t ICU4XDateTime_days_in_year(const DateTime* self);
    
    Calendar* ICU4XDateTime_calendar(const DateTime* self);
    
    
    void ICU4XDateTime_destroy(DateTime* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<DateTime>, CalendarError> DateTime::create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar& calendar) {
  auto result = capi::ICU4XDateTime_create_from_iso_in_calendar(year,
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
  auto result = capi::ICU4XDateTime_create_from_codes_in_calendar(era_code.data(),
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
  auto result = capi::ICU4XDateTime_create_from_date_and_time(date.AsFFI(),
    time.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline std::unique_ptr<Date> DateTime::date() const {
  auto result = capi::ICU4XDateTime_date(this->AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline std::unique_ptr<Time> DateTime::time() const {
  auto result = capi::ICU4XDateTime_time(this->AsFFI());
  return std::unique_ptr<Time>(Time::FromFFI(result));
}

inline std::unique_ptr<IsoDateTime> DateTime::to_iso() const {
  auto result = capi::ICU4XDateTime_to_iso(this->AsFFI());
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<DateTime> DateTime::to_calendar(const Calendar& calendar) const {
  auto result = capi::ICU4XDateTime_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline uint8_t DateTime::hour() const {
  auto result = capi::ICU4XDateTime_hour(this->AsFFI());
  return result;
}

inline uint8_t DateTime::minute() const {
  auto result = capi::ICU4XDateTime_minute(this->AsFFI());
  return result;
}

inline uint8_t DateTime::second() const {
  auto result = capi::ICU4XDateTime_second(this->AsFFI());
  return result;
}

inline uint32_t DateTime::nanosecond() const {
  auto result = capi::ICU4XDateTime_nanosecond(this->AsFFI());
  return result;
}

inline uint16_t DateTime::day_of_year() const {
  auto result = capi::ICU4XDateTime_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t DateTime::day_of_month() const {
  auto result = capi::ICU4XDateTime_day_of_month(this->AsFFI());
  return result;
}

inline IsoWeekday DateTime::day_of_week() const {
  auto result = capi::ICU4XDateTime_day_of_week(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t DateTime::week_of_month(IsoWeekday first_weekday) const {
  auto result = capi::ICU4XDateTime_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf DateTime::week_of_year(const WeekCalculator& calculator) const {
  auto result = capi::ICU4XDateTime_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t DateTime::ordinal_month() const {
  auto result = capi::ICU4XDateTime_ordinal_month(this->AsFFI());
  return result;
}

inline std::string DateTime::month_code() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDateTime_month_code(this->AsFFI(),
    &write);
  return output;
}

inline int32_t DateTime::year_in_era() const {
  auto result = capi::ICU4XDateTime_year_in_era(this->AsFFI());
  return result;
}

inline std::string DateTime::era() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDateTime_era(this->AsFFI(),
    &write);
  return output;
}

inline uint8_t DateTime::months_in_year() const {
  auto result = capi::ICU4XDateTime_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t DateTime::days_in_month() const {
  auto result = capi::ICU4XDateTime_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t DateTime::days_in_year() const {
  auto result = capi::ICU4XDateTime_days_in_year(this->AsFFI());
  return result;
}

inline std::unique_ptr<Calendar> DateTime::calendar() const {
  auto result = capi::ICU4XDateTime_calendar(this->AsFFI());
  return std::unique_ptr<Calendar>(Calendar::FromFFI(result));
}

inline const capi::DateTime* DateTime::AsFFI() const {
  return reinterpret_cast<const capi::DateTime*>(this);
}

inline capi::DateTime* DateTime::AsFFI() {
  return reinterpret_cast<capi::DateTime*>(this);
}

inline const DateTime* DateTime::FromFFI(const capi::DateTime* ptr) {
  return reinterpret_cast<const DateTime*>(ptr);
}

inline DateTime* DateTime::FromFFI(capi::DateTime* ptr) {
  return reinterpret_cast<DateTime*>(ptr);
}

inline void DateTime::operator delete(void* ptr) {
  capi::ICU4XDateTime_destroy(reinterpret_cast<capi::DateTime*>(ptr));
}


#endif // DateTime_HPP

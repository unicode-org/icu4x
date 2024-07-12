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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XDateTime_create_from_iso_in_calendar_result {union {diplomat::capi::DateTime* ok; diplomat::capi::CalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_iso_in_calendar_result;
    ICU4XDateTime_create_from_iso_in_calendar_result ICU4XDateTime_create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const diplomat::capi::Calendar* calendar);
    
    typedef struct ICU4XDateTime_create_from_codes_in_calendar_result {union {diplomat::capi::DateTime* ok; diplomat::capi::CalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_codes_in_calendar_result;
    ICU4XDateTime_create_from_codes_in_calendar_result ICU4XDateTime_create_from_codes_in_calendar(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const diplomat::capi::Calendar* calendar);
    
    diplomat::capi::DateTime* ICU4XDateTime_create_from_date_and_time(const diplomat::capi::Date* date, const diplomat::capi::Time* time);
    
    diplomat::capi::Date* ICU4XDateTime_date(const diplomat::capi::DateTime* self);
    
    diplomat::capi::Time* ICU4XDateTime_time(const diplomat::capi::DateTime* self);
    
    diplomat::capi::IsoDateTime* ICU4XDateTime_to_iso(const diplomat::capi::DateTime* self);
    
    diplomat::capi::DateTime* ICU4XDateTime_to_calendar(const diplomat::capi::DateTime* self, const diplomat::capi::Calendar* calendar);
    
    uint8_t ICU4XDateTime_hour(const diplomat::capi::DateTime* self);
    
    uint8_t ICU4XDateTime_minute(const diplomat::capi::DateTime* self);
    
    uint8_t ICU4XDateTime_second(const diplomat::capi::DateTime* self);
    
    uint32_t ICU4XDateTime_nanosecond(const diplomat::capi::DateTime* self);
    
    uint16_t ICU4XDateTime_day_of_year(const diplomat::capi::DateTime* self);
    
    uint32_t ICU4XDateTime_day_of_month(const diplomat::capi::DateTime* self);
    
    diplomat::capi::IsoWeekday ICU4XDateTime_day_of_week(const diplomat::capi::DateTime* self);
    
    uint32_t ICU4XDateTime_week_of_month(const diplomat::capi::DateTime* self, diplomat::capi::IsoWeekday first_weekday);
    
    diplomat::capi::WeekOf ICU4XDateTime_week_of_year(const diplomat::capi::DateTime* self, const diplomat::capi::WeekCalculator* calculator);
    
    uint32_t ICU4XDateTime_ordinal_month(const diplomat::capi::DateTime* self);
    
    void ICU4XDateTime_month_code(const diplomat::capi::DateTime* self, diplomat::capi::DiplomatWrite* write);
    
    int32_t ICU4XDateTime_year_in_era(const diplomat::capi::DateTime* self);
    
    void ICU4XDateTime_era(const diplomat::capi::DateTime* self, diplomat::capi::DiplomatWrite* write);
    
    uint8_t ICU4XDateTime_months_in_year(const diplomat::capi::DateTime* self);
    
    uint8_t ICU4XDateTime_days_in_month(const diplomat::capi::DateTime* self);
    
    uint16_t ICU4XDateTime_days_in_year(const diplomat::capi::DateTime* self);
    
    diplomat::capi::Calendar* ICU4XDateTime_calendar(const diplomat::capi::DateTime* self);
    
    
    void ICU4XDateTime_destroy(DateTime* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<DateTime>, CalendarError> DateTime::create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar& calendar) {
  auto result = diplomat::capi::ICU4XDateTime_create_from_iso_in_calendar(year,
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
  auto result = diplomat::capi::ICU4XDateTime_create_from_codes_in_calendar(era_code.data(),
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
  auto result = diplomat::capi::ICU4XDateTime_create_from_date_and_time(date.AsFFI(),
    time.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline std::unique_ptr<Date> DateTime::date() const {
  auto result = diplomat::capi::ICU4XDateTime_date(this->AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline std::unique_ptr<Time> DateTime::time() const {
  auto result = diplomat::capi::ICU4XDateTime_time(this->AsFFI());
  return std::unique_ptr<Time>(Time::FromFFI(result));
}

inline std::unique_ptr<IsoDateTime> DateTime::to_iso() const {
  auto result = diplomat::capi::ICU4XDateTime_to_iso(this->AsFFI());
  return std::unique_ptr<IsoDateTime>(IsoDateTime::FromFFI(result));
}

inline std::unique_ptr<DateTime> DateTime::to_calendar(const Calendar& calendar) const {
  auto result = diplomat::capi::ICU4XDateTime_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<DateTime>(DateTime::FromFFI(result));
}

inline uint8_t DateTime::hour() const {
  auto result = diplomat::capi::ICU4XDateTime_hour(this->AsFFI());
  return result;
}

inline uint8_t DateTime::minute() const {
  auto result = diplomat::capi::ICU4XDateTime_minute(this->AsFFI());
  return result;
}

inline uint8_t DateTime::second() const {
  auto result = diplomat::capi::ICU4XDateTime_second(this->AsFFI());
  return result;
}

inline uint32_t DateTime::nanosecond() const {
  auto result = diplomat::capi::ICU4XDateTime_nanosecond(this->AsFFI());
  return result;
}

inline uint16_t DateTime::day_of_year() const {
  auto result = diplomat::capi::ICU4XDateTime_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t DateTime::day_of_month() const {
  auto result = diplomat::capi::ICU4XDateTime_day_of_month(this->AsFFI());
  return result;
}

inline IsoWeekday DateTime::day_of_week() const {
  auto result = diplomat::capi::ICU4XDateTime_day_of_week(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t DateTime::week_of_month(IsoWeekday first_weekday) const {
  auto result = diplomat::capi::ICU4XDateTime_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf DateTime::week_of_year(const WeekCalculator& calculator) const {
  auto result = diplomat::capi::ICU4XDateTime_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t DateTime::ordinal_month() const {
  auto result = diplomat::capi::ICU4XDateTime_ordinal_month(this->AsFFI());
  return result;
}

inline std::string DateTime::month_code() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XDateTime_month_code(this->AsFFI(),
    &write);
  return output;
}

inline int32_t DateTime::year_in_era() const {
  auto result = diplomat::capi::ICU4XDateTime_year_in_era(this->AsFFI());
  return result;
}

inline std::string DateTime::era() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XDateTime_era(this->AsFFI(),
    &write);
  return output;
}

inline uint8_t DateTime::months_in_year() const {
  auto result = diplomat::capi::ICU4XDateTime_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t DateTime::days_in_month() const {
  auto result = diplomat::capi::ICU4XDateTime_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t DateTime::days_in_year() const {
  auto result = diplomat::capi::ICU4XDateTime_days_in_year(this->AsFFI());
  return result;
}

inline std::unique_ptr<Calendar> DateTime::calendar() const {
  auto result = diplomat::capi::ICU4XDateTime_calendar(this->AsFFI());
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
  diplomat::capi::ICU4XDateTime_destroy(reinterpret_cast<diplomat::capi::DateTime*>(ptr));
}


#endif // DateTime_HPP

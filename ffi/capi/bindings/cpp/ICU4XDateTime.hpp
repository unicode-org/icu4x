#ifndef ICU4XDateTime_HPP
#define ICU4XDateTime_HPP

#include "ICU4XDateTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendar.hpp"
#include "ICU4XCalendarError.hpp"
#include "ICU4XDate.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XIsoWeekday.hpp"
#include "ICU4XTime.hpp"
#include "ICU4XWeekCalculator.hpp"
#include "ICU4XWeekOf.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XDateTime_create_from_iso_in_calendar_result {union {ICU4XDateTime* ok; ICU4XCalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_iso_in_calendar_result;
    ICU4XDateTime_create_from_iso_in_calendar_result ICU4XDateTime_create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar* calendar);
    
    typedef struct ICU4XDateTime_create_from_codes_in_calendar_result {union {ICU4XDateTime* ok; ICU4XCalendarError err;}; bool is_ok;} ICU4XDateTime_create_from_codes_in_calendar_result;
    ICU4XDateTime_create_from_codes_in_calendar_result ICU4XDateTime_create_from_codes_in_calendar(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar* calendar);
    
    ICU4XDateTime* ICU4XDateTime_create_from_date_and_time(const ICU4XDate* date, const ICU4XTime* time);
    
    ICU4XDate* ICU4XDateTime_date(const ICU4XDateTime* self);
    
    ICU4XTime* ICU4XDateTime_time(const ICU4XDateTime* self);
    
    ICU4XIsoDateTime* ICU4XDateTime_to_iso(const ICU4XDateTime* self);
    
    ICU4XDateTime* ICU4XDateTime_to_calendar(const ICU4XDateTime* self, const ICU4XCalendar* calendar);
    
    uint8_t ICU4XDateTime_hour(const ICU4XDateTime* self);
    
    uint8_t ICU4XDateTime_minute(const ICU4XDateTime* self);
    
    uint8_t ICU4XDateTime_second(const ICU4XDateTime* self);
    
    uint32_t ICU4XDateTime_nanosecond(const ICU4XDateTime* self);
    
    uint16_t ICU4XDateTime_day_of_year(const ICU4XDateTime* self);
    
    uint32_t ICU4XDateTime_day_of_month(const ICU4XDateTime* self);
    
    ICU4XIsoWeekday ICU4XDateTime_day_of_week(const ICU4XDateTime* self);
    
    uint32_t ICU4XDateTime_week_of_month(const ICU4XDateTime* self, ICU4XIsoWeekday first_weekday);
    
    ICU4XWeekOf ICU4XDateTime_week_of_year(const ICU4XDateTime* self, const ICU4XWeekCalculator* calculator);
    
    uint32_t ICU4XDateTime_ordinal_month(const ICU4XDateTime* self);
    
    void ICU4XDateTime_month_code(const ICU4XDateTime* self, DiplomatWrite* write);
    
    int32_t ICU4XDateTime_year_in_era(const ICU4XDateTime* self);
    
    void ICU4XDateTime_era(const ICU4XDateTime* self, DiplomatWrite* write);
    
    uint8_t ICU4XDateTime_months_in_year(const ICU4XDateTime* self);
    
    uint8_t ICU4XDateTime_days_in_month(const ICU4XDateTime* self);
    
    uint16_t ICU4XDateTime_days_in_year(const ICU4XDateTime* self);
    
    ICU4XCalendar* ICU4XDateTime_calendar(const ICU4XDateTime* self);
    
    
    void ICU4XDateTime_destroy(ICU4XDateTime* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XCalendarError> ICU4XDateTime::create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar) {
  auto result = capi::ICU4XDateTime_create_from_iso_in_calendar(year,
    month,
    day,
    hour,
    minute,
    second,
    nanosecond,
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XCalendarError>(diplomat::Ok<std::unique_ptr<ICU4XDateTime>>(std::unique_ptr<ICU4XDateTime>(ICU4XDateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XCalendarError>(diplomat::Err<ICU4XCalendarError>(ICU4XCalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XCalendarError> ICU4XDateTime::create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar) {
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
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XCalendarError>(diplomat::Ok<std::unique_ptr<ICU4XDateTime>>(std::unique_ptr<ICU4XDateTime>(ICU4XDateTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XCalendarError>(diplomat::Err<ICU4XCalendarError>(ICU4XCalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XDateTime> ICU4XDateTime::create_from_date_and_time(const ICU4XDate& date, const ICU4XTime& time) {
  auto result = capi::ICU4XDateTime_create_from_date_and_time(date.AsFFI(),
    time.AsFFI());
  return std::unique_ptr<ICU4XDateTime>(ICU4XDateTime::FromFFI(result));
}

inline std::unique_ptr<ICU4XDate> ICU4XDateTime::date() const {
  auto result = capi::ICU4XDateTime_date(this->AsFFI());
  return std::unique_ptr<ICU4XDate>(ICU4XDate::FromFFI(result));
}

inline std::unique_ptr<ICU4XTime> ICU4XDateTime::time() const {
  auto result = capi::ICU4XDateTime_time(this->AsFFI());
  return std::unique_ptr<ICU4XTime>(ICU4XTime::FromFFI(result));
}

inline std::unique_ptr<ICU4XIsoDateTime> ICU4XDateTime::to_iso() const {
  auto result = capi::ICU4XDateTime_to_iso(this->AsFFI());
  return std::unique_ptr<ICU4XIsoDateTime>(ICU4XIsoDateTime::FromFFI(result));
}

inline std::unique_ptr<ICU4XDateTime> ICU4XDateTime::to_calendar(const ICU4XCalendar& calendar) const {
  auto result = capi::ICU4XDateTime_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<ICU4XDateTime>(ICU4XDateTime::FromFFI(result));
}

inline uint8_t ICU4XDateTime::hour() const {
  auto result = capi::ICU4XDateTime_hour(this->AsFFI());
  return result;
}

inline uint8_t ICU4XDateTime::minute() const {
  auto result = capi::ICU4XDateTime_minute(this->AsFFI());
  return result;
}

inline uint8_t ICU4XDateTime::second() const {
  auto result = capi::ICU4XDateTime_second(this->AsFFI());
  return result;
}

inline uint32_t ICU4XDateTime::nanosecond() const {
  auto result = capi::ICU4XDateTime_nanosecond(this->AsFFI());
  return result;
}

inline uint16_t ICU4XDateTime::day_of_year() const {
  auto result = capi::ICU4XDateTime_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t ICU4XDateTime::day_of_month() const {
  auto result = capi::ICU4XDateTime_day_of_month(this->AsFFI());
  return result;
}

inline ICU4XIsoWeekday ICU4XDateTime::day_of_week() const {
  auto result = capi::ICU4XDateTime_day_of_week(this->AsFFI());
  return ICU4XIsoWeekday::FromFFI(result);
}

inline uint32_t ICU4XDateTime::week_of_month(ICU4XIsoWeekday first_weekday) const {
  auto result = capi::ICU4XDateTime_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline ICU4XWeekOf ICU4XDateTime::week_of_year(const ICU4XWeekCalculator& calculator) const {
  auto result = capi::ICU4XDateTime_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return ICU4XWeekOf::FromFFI(result);
}

inline uint32_t ICU4XDateTime::ordinal_month() const {
  auto result = capi::ICU4XDateTime_ordinal_month(this->AsFFI());
  return result;
}

inline std::string ICU4XDateTime::month_code() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDateTime_month_code(this->AsFFI(),
    &write);
  return output;
}

inline int32_t ICU4XDateTime::year_in_era() const {
  auto result = capi::ICU4XDateTime_year_in_era(this->AsFFI());
  return result;
}

inline std::string ICU4XDateTime::era() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDateTime_era(this->AsFFI(),
    &write);
  return output;
}

inline uint8_t ICU4XDateTime::months_in_year() const {
  auto result = capi::ICU4XDateTime_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t ICU4XDateTime::days_in_month() const {
  auto result = capi::ICU4XDateTime_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t ICU4XDateTime::days_in_year() const {
  auto result = capi::ICU4XDateTime_days_in_year(this->AsFFI());
  return result;
}

inline std::unique_ptr<ICU4XCalendar> ICU4XDateTime::calendar() const {
  auto result = capi::ICU4XDateTime_calendar(this->AsFFI());
  return std::unique_ptr<ICU4XCalendar>(ICU4XCalendar::FromFFI(result));
}

inline const capi::ICU4XDateTime* ICU4XDateTime::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDateTime*>(this);
}

inline capi::ICU4XDateTime* ICU4XDateTime::AsFFI() {
  return reinterpret_cast<capi::ICU4XDateTime*>(this);
}

inline const ICU4XDateTime* ICU4XDateTime::FromFFI(const capi::ICU4XDateTime* ptr) {
  return reinterpret_cast<const ICU4XDateTime*>(ptr);
}

inline ICU4XDateTime* ICU4XDateTime::FromFFI(capi::ICU4XDateTime* ptr) {
  return reinterpret_cast<ICU4XDateTime*>(ptr);
}

inline void ICU4XDateTime::operator delete(void* ptr) {
  capi::ICU4XDateTime_destroy(reinterpret_cast<capi::ICU4XDateTime*>(ptr));
}


#endif // ICU4XDateTime_HPP

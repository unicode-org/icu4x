#ifndef Date_HPP
#define Date_HPP

#include "Date.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Calendar.hpp"
#include "CalendarError.hpp"
#include "IsoDate.hpp"
#include "IsoWeekday.hpp"
#include "WeekCalculator.hpp"
#include "WeekOf.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XDate_create_from_iso_in_calendar_result {union {diplomat::capi::Date* ok; diplomat::capi::CalendarError err;}; bool is_ok;} ICU4XDate_create_from_iso_in_calendar_result;
    ICU4XDate_create_from_iso_in_calendar_result ICU4XDate_create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, const diplomat::capi::Calendar* calendar);
    
    typedef struct ICU4XDate_create_from_codes_in_calendar_result {union {diplomat::capi::Date* ok; diplomat::capi::CalendarError err;}; bool is_ok;} ICU4XDate_create_from_codes_in_calendar_result;
    ICU4XDate_create_from_codes_in_calendar_result ICU4XDate_create_from_codes_in_calendar(const char* era_code_data, size_t era_code_len, int32_t year, const char* month_code_data, size_t month_code_len, uint8_t day, const diplomat::capi::Calendar* calendar);
    
    diplomat::capi::Date* ICU4XDate_to_calendar(const diplomat::capi::Date* self, const diplomat::capi::Calendar* calendar);
    
    diplomat::capi::IsoDate* ICU4XDate_to_iso(const diplomat::capi::Date* self);
    
    uint16_t ICU4XDate_day_of_year(const diplomat::capi::Date* self);
    
    uint32_t ICU4XDate_day_of_month(const diplomat::capi::Date* self);
    
    diplomat::capi::IsoWeekday ICU4XDate_day_of_week(const diplomat::capi::Date* self);
    
    uint32_t ICU4XDate_week_of_month(const diplomat::capi::Date* self, diplomat::capi::IsoWeekday first_weekday);
    
    diplomat::capi::WeekOf ICU4XDate_week_of_year(const diplomat::capi::Date* self, const diplomat::capi::WeekCalculator* calculator);
    
    uint32_t ICU4XDate_ordinal_month(const diplomat::capi::Date* self);
    
    void ICU4XDate_month_code(const diplomat::capi::Date* self, diplomat::capi::DiplomatWrite* write);
    
    int32_t ICU4XDate_year_in_era(const diplomat::capi::Date* self);
    
    void ICU4XDate_era(const diplomat::capi::Date* self, diplomat::capi::DiplomatWrite* write);
    
    uint8_t ICU4XDate_months_in_year(const diplomat::capi::Date* self);
    
    uint8_t ICU4XDate_days_in_month(const diplomat::capi::Date* self);
    
    uint16_t ICU4XDate_days_in_year(const diplomat::capi::Date* self);
    
    diplomat::capi::Calendar* ICU4XDate_calendar(const diplomat::capi::Date* self);
    
    
    void ICU4XDate_destroy(Date* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Date>, CalendarError> Date::create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, const Calendar& calendar) {
  auto result = diplomat::capi::ICU4XDate_create_from_iso_in_calendar(year,
    month,
    day,
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Date>, CalendarError>(diplomat::Ok<std::unique_ptr<Date>>(std::unique_ptr<Date>(Date::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Date>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<Date>, CalendarError> Date::create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, const Calendar& calendar) {
  auto result = diplomat::capi::ICU4XDate_create_from_codes_in_calendar(era_code.data(),
    era_code.size(),
    year,
    month_code.data(),
    month_code.size(),
    day,
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Date>, CalendarError>(diplomat::Ok<std::unique_ptr<Date>>(std::unique_ptr<Date>(Date::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Date>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<Date> Date::to_calendar(const Calendar& calendar) const {
  auto result = diplomat::capi::ICU4XDate_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline std::unique_ptr<IsoDate> Date::to_iso() const {
  auto result = diplomat::capi::ICU4XDate_to_iso(this->AsFFI());
  return std::unique_ptr<IsoDate>(IsoDate::FromFFI(result));
}

inline uint16_t Date::day_of_year() const {
  auto result = diplomat::capi::ICU4XDate_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t Date::day_of_month() const {
  auto result = diplomat::capi::ICU4XDate_day_of_month(this->AsFFI());
  return result;
}

inline IsoWeekday Date::day_of_week() const {
  auto result = diplomat::capi::ICU4XDate_day_of_week(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t Date::week_of_month(IsoWeekday first_weekday) const {
  auto result = diplomat::capi::ICU4XDate_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf Date::week_of_year(const WeekCalculator& calculator) const {
  auto result = diplomat::capi::ICU4XDate_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t Date::ordinal_month() const {
  auto result = diplomat::capi::ICU4XDate_ordinal_month(this->AsFFI());
  return result;
}

inline std::string Date::month_code() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XDate_month_code(this->AsFFI(),
    &write);
  return output;
}

inline int32_t Date::year_in_era() const {
  auto result = diplomat::capi::ICU4XDate_year_in_era(this->AsFFI());
  return result;
}

inline std::string Date::era() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XDate_era(this->AsFFI(),
    &write);
  return output;
}

inline uint8_t Date::months_in_year() const {
  auto result = diplomat::capi::ICU4XDate_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t Date::days_in_month() const {
  auto result = diplomat::capi::ICU4XDate_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t Date::days_in_year() const {
  auto result = diplomat::capi::ICU4XDate_days_in_year(this->AsFFI());
  return result;
}

inline std::unique_ptr<Calendar> Date::calendar() const {
  auto result = diplomat::capi::ICU4XDate_calendar(this->AsFFI());
  return std::unique_ptr<Calendar>(Calendar::FromFFI(result));
}

inline const diplomat::capi::Date* Date::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Date*>(this);
}

inline diplomat::capi::Date* Date::AsFFI() {
  return reinterpret_cast<diplomat::capi::Date*>(this);
}

inline const Date* Date::FromFFI(const diplomat::capi::Date* ptr) {
  return reinterpret_cast<const Date*>(ptr);
}

inline Date* Date::FromFFI(diplomat::capi::Date* ptr) {
  return reinterpret_cast<Date*>(ptr);
}

inline void Date::operator delete(void* ptr) {
  diplomat::capi::ICU4XDate_destroy(reinterpret_cast<diplomat::capi::Date*>(ptr));
}


#endif // Date_HPP

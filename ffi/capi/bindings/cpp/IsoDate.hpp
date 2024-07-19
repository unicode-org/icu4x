#ifndef IsoDate_HPP
#define IsoDate_HPP

#include "IsoDate.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "Calendar.hpp"
#include "CalendarError.hpp"
#include "CalendarParseError.hpp"
#include "Date.hpp"
#include "IsoWeekday.hpp"
#include "WeekCalculator.hpp"
#include "WeekOf.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_IsoDate_create_mv1_result {union {diplomat::capi::IsoDate* ok; diplomat::capi::CalendarError err;}; bool is_ok;} icu4x_IsoDate_create_mv1_result;
    icu4x_IsoDate_create_mv1_result icu4x_IsoDate_create_mv1(int32_t year, uint8_t month, uint8_t day);
    
    typedef struct icu4x_IsoDate_from_string_mv1_result {union {diplomat::capi::IsoDate* ok; diplomat::capi::CalendarParseError err;}; bool is_ok;} icu4x_IsoDate_from_string_mv1_result;
    icu4x_IsoDate_from_string_mv1_result icu4x_IsoDate_from_string_mv1(const char* v_data, size_t v_len);
    
    diplomat::capi::IsoDate* icu4x_IsoDate_unix_epoch_mv1(void);
    
    diplomat::capi::Date* icu4x_IsoDate_to_calendar_mv1(const diplomat::capi::IsoDate* self, const diplomat::capi::Calendar* calendar);
    
    diplomat::capi::Date* icu4x_IsoDate_to_any_mv1(const diplomat::capi::IsoDate* self);
    
    uint16_t icu4x_IsoDate_day_of_year_mv1(const diplomat::capi::IsoDate* self);
    
    uint32_t icu4x_IsoDate_day_of_month_mv1(const diplomat::capi::IsoDate* self);
    
    diplomat::capi::IsoWeekday icu4x_IsoDate_day_of_week_mv1(const diplomat::capi::IsoDate* self);
    
    uint32_t icu4x_IsoDate_week_of_month_mv1(const diplomat::capi::IsoDate* self, diplomat::capi::IsoWeekday first_weekday);
    
    diplomat::capi::WeekOf icu4x_IsoDate_week_of_year_mv1(const diplomat::capi::IsoDate* self, const diplomat::capi::WeekCalculator* calculator);
    
    uint32_t icu4x_IsoDate_month_mv1(const diplomat::capi::IsoDate* self);
    
    int32_t icu4x_IsoDate_year_mv1(const diplomat::capi::IsoDate* self);
    
    bool icu4x_IsoDate_is_in_leap_year_mv1(const diplomat::capi::IsoDate* self);
    
    uint8_t icu4x_IsoDate_months_in_year_mv1(const diplomat::capi::IsoDate* self);
    
    uint8_t icu4x_IsoDate_days_in_month_mv1(const diplomat::capi::IsoDate* self);
    
    uint16_t icu4x_IsoDate_days_in_year_mv1(const diplomat::capi::IsoDate* self);
    
    
    void icu4x_IsoDate_destroy_mv1(IsoDate* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<IsoDate>, CalendarError> IsoDate::create(int32_t year, uint8_t month, uint8_t day) {
  auto result = diplomat::capi::icu4x_IsoDate_create_mv1(year,
    month,
    day);
  return result.is_ok ? diplomat::result<std::unique_ptr<IsoDate>, CalendarError>(diplomat::Ok<std::unique_ptr<IsoDate>>(std::unique_ptr<IsoDate>(IsoDate::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<IsoDate>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<IsoDate>, CalendarParseError> IsoDate::from_string(std::string_view v) {
  auto result = diplomat::capi::icu4x_IsoDate_from_string_mv1(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<IsoDate>, CalendarParseError>(diplomat::Ok<std::unique_ptr<IsoDate>>(std::unique_ptr<IsoDate>(IsoDate::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<IsoDate>, CalendarParseError>(diplomat::Err<CalendarParseError>(CalendarParseError::FromFFI(result.err)));
}

inline std::unique_ptr<IsoDate> IsoDate::unix_epoch() {
  auto result = diplomat::capi::icu4x_IsoDate_unix_epoch_mv1();
  return std::unique_ptr<IsoDate>(IsoDate::FromFFI(result));
}

inline std::unique_ptr<Date> IsoDate::to_calendar(const Calendar& calendar) const {
  auto result = diplomat::capi::icu4x_IsoDate_to_calendar_mv1(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline std::unique_ptr<Date> IsoDate::to_any() const {
  auto result = diplomat::capi::icu4x_IsoDate_to_any_mv1(this->AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline uint16_t IsoDate::day_of_year() const {
  auto result = diplomat::capi::icu4x_IsoDate_day_of_year_mv1(this->AsFFI());
  return result;
}

inline uint32_t IsoDate::day_of_month() const {
  auto result = diplomat::capi::icu4x_IsoDate_day_of_month_mv1(this->AsFFI());
  return result;
}

inline IsoWeekday IsoDate::day_of_week() const {
  auto result = diplomat::capi::icu4x_IsoDate_day_of_week_mv1(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t IsoDate::week_of_month(IsoWeekday first_weekday) const {
  auto result = diplomat::capi::icu4x_IsoDate_week_of_month_mv1(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf IsoDate::week_of_year(const WeekCalculator& calculator) const {
  auto result = diplomat::capi::icu4x_IsoDate_week_of_year_mv1(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t IsoDate::month() const {
  auto result = diplomat::capi::icu4x_IsoDate_month_mv1(this->AsFFI());
  return result;
}

inline int32_t IsoDate::year() const {
  auto result = diplomat::capi::icu4x_IsoDate_year_mv1(this->AsFFI());
  return result;
}

inline bool IsoDate::is_in_leap_year() const {
  auto result = diplomat::capi::icu4x_IsoDate_is_in_leap_year_mv1(this->AsFFI());
  return result;
}

inline uint8_t IsoDate::months_in_year() const {
  auto result = diplomat::capi::icu4x_IsoDate_months_in_year_mv1(this->AsFFI());
  return result;
}

inline uint8_t IsoDate::days_in_month() const {
  auto result = diplomat::capi::icu4x_IsoDate_days_in_month_mv1(this->AsFFI());
  return result;
}

inline uint16_t IsoDate::days_in_year() const {
  auto result = diplomat::capi::icu4x_IsoDate_days_in_year_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::IsoDate* IsoDate::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::IsoDate*>(this);
}

inline diplomat::capi::IsoDate* IsoDate::AsFFI() {
  return reinterpret_cast<diplomat::capi::IsoDate*>(this);
}

inline const IsoDate* IsoDate::FromFFI(const diplomat::capi::IsoDate* ptr) {
  return reinterpret_cast<const IsoDate*>(ptr);
}

inline IsoDate* IsoDate::FromFFI(diplomat::capi::IsoDate* ptr) {
  return reinterpret_cast<IsoDate*>(ptr);
}

inline void IsoDate::operator delete(void* ptr) {
  diplomat::capi::icu4x_IsoDate_destroy_mv1(reinterpret_cast<diplomat::capi::IsoDate*>(ptr));
}


#endif // IsoDate_HPP

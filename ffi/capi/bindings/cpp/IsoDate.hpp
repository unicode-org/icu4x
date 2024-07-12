#ifndef IsoDate_HPP
#define IsoDate_HPP

#include "IsoDate.d.hpp"

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
#include "IsoWeekday.hpp"
#include "WeekCalculator.hpp"
#include "WeekOf.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XIsoDate_create_result {union {diplomat::capi::IsoDate* ok; diplomat::capi::CalendarError err;}; bool is_ok;} ICU4XIsoDate_create_result;
    ICU4XIsoDate_create_result ICU4XIsoDate_create(int32_t year, uint8_t month, uint8_t day);
    
    diplomat::capi::IsoDate* ICU4XIsoDate_create_for_unix_epoch();
    
    diplomat::capi::Date* ICU4XIsoDate_to_calendar(const diplomat::capi::IsoDate* self, const diplomat::capi::Calendar* calendar);
    
    diplomat::capi::Date* ICU4XIsoDate_to_any(const diplomat::capi::IsoDate* self);
    
    uint16_t ICU4XIsoDate_day_of_year(const diplomat::capi::IsoDate* self);
    
    uint32_t ICU4XIsoDate_day_of_month(const diplomat::capi::IsoDate* self);
    
    diplomat::capi::IsoWeekday ICU4XIsoDate_day_of_week(const diplomat::capi::IsoDate* self);
    
    uint32_t ICU4XIsoDate_week_of_month(const diplomat::capi::IsoDate* self, diplomat::capi::IsoWeekday first_weekday);
    
    diplomat::capi::WeekOf ICU4XIsoDate_week_of_year(const diplomat::capi::IsoDate* self, const diplomat::capi::WeekCalculator* calculator);
    
    uint32_t ICU4XIsoDate_month(const diplomat::capi::IsoDate* self);
    
    int32_t ICU4XIsoDate_year(const diplomat::capi::IsoDate* self);
    
    bool ICU4XIsoDate_is_in_leap_year(const diplomat::capi::IsoDate* self);
    
    uint8_t ICU4XIsoDate_months_in_year(const diplomat::capi::IsoDate* self);
    
    uint8_t ICU4XIsoDate_days_in_month(const diplomat::capi::IsoDate* self);
    
    uint16_t ICU4XIsoDate_days_in_year(const diplomat::capi::IsoDate* self);
    
    
    void ICU4XIsoDate_destroy(IsoDate* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<IsoDate>, CalendarError> IsoDate::create(int32_t year, uint8_t month, uint8_t day) {
  auto result = diplomat::capi::ICU4XIsoDate_create(year,
    month,
    day);
  return result.is_ok ? diplomat::result<std::unique_ptr<IsoDate>, CalendarError>(diplomat::Ok<std::unique_ptr<IsoDate>>(std::unique_ptr<IsoDate>(IsoDate::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<IsoDate>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<IsoDate> IsoDate::create_for_unix_epoch() {
  auto result = diplomat::capi::ICU4XIsoDate_create_for_unix_epoch();
  return std::unique_ptr<IsoDate>(IsoDate::FromFFI(result));
}

inline std::unique_ptr<Date> IsoDate::to_calendar(const Calendar& calendar) const {
  auto result = diplomat::capi::ICU4XIsoDate_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline std::unique_ptr<Date> IsoDate::to_any() const {
  auto result = diplomat::capi::ICU4XIsoDate_to_any(this->AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline uint16_t IsoDate::day_of_year() const {
  auto result = diplomat::capi::ICU4XIsoDate_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t IsoDate::day_of_month() const {
  auto result = diplomat::capi::ICU4XIsoDate_day_of_month(this->AsFFI());
  return result;
}

inline IsoWeekday IsoDate::day_of_week() const {
  auto result = diplomat::capi::ICU4XIsoDate_day_of_week(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t IsoDate::week_of_month(IsoWeekday first_weekday) const {
  auto result = diplomat::capi::ICU4XIsoDate_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf IsoDate::week_of_year(const WeekCalculator& calculator) const {
  auto result = diplomat::capi::ICU4XIsoDate_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t IsoDate::month() const {
  auto result = diplomat::capi::ICU4XIsoDate_month(this->AsFFI());
  return result;
}

inline int32_t IsoDate::year() const {
  auto result = diplomat::capi::ICU4XIsoDate_year(this->AsFFI());
  return result;
}

inline bool IsoDate::is_in_leap_year() const {
  auto result = diplomat::capi::ICU4XIsoDate_is_in_leap_year(this->AsFFI());
  return result;
}

inline uint8_t IsoDate::months_in_year() const {
  auto result = diplomat::capi::ICU4XIsoDate_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t IsoDate::days_in_month() const {
  auto result = diplomat::capi::ICU4XIsoDate_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t IsoDate::days_in_year() const {
  auto result = diplomat::capi::ICU4XIsoDate_days_in_year(this->AsFFI());
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
  diplomat::capi::ICU4XIsoDate_destroy(reinterpret_cast<diplomat::capi::IsoDate*>(ptr));
}


#endif // IsoDate_HPP

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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XIsoDate_create_result {union {IsoDate* ok; CalendarError err;}; bool is_ok;} ICU4XIsoDate_create_result;
    ICU4XIsoDate_create_result ICU4XIsoDate_create(int32_t year, uint8_t month, uint8_t day);
    
    IsoDate* ICU4XIsoDate_create_for_unix_epoch();
    
    Date* ICU4XIsoDate_to_calendar(const IsoDate* self, const Calendar* calendar);
    
    Date* ICU4XIsoDate_to_any(const IsoDate* self);
    
    uint16_t ICU4XIsoDate_day_of_year(const IsoDate* self);
    
    uint32_t ICU4XIsoDate_day_of_month(const IsoDate* self);
    
    IsoWeekday ICU4XIsoDate_day_of_week(const IsoDate* self);
    
    uint32_t ICU4XIsoDate_week_of_month(const IsoDate* self, IsoWeekday first_weekday);
    
    WeekOf ICU4XIsoDate_week_of_year(const IsoDate* self, const WeekCalculator* calculator);
    
    uint32_t ICU4XIsoDate_month(const IsoDate* self);
    
    int32_t ICU4XIsoDate_year(const IsoDate* self);
    
    bool ICU4XIsoDate_is_in_leap_year(const IsoDate* self);
    
    uint8_t ICU4XIsoDate_months_in_year(const IsoDate* self);
    
    uint8_t ICU4XIsoDate_days_in_month(const IsoDate* self);
    
    uint16_t ICU4XIsoDate_days_in_year(const IsoDate* self);
    
    
    void ICU4XIsoDate_destroy(IsoDate* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<IsoDate>, CalendarError> IsoDate::create(int32_t year, uint8_t month, uint8_t day) {
  auto result = capi::ICU4XIsoDate_create(year,
    month,
    day);
  return result.is_ok ? diplomat::result<std::unique_ptr<IsoDate>, CalendarError>(diplomat::Ok<std::unique_ptr<IsoDate>>(std::unique_ptr<IsoDate>(IsoDate::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<IsoDate>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline std::unique_ptr<IsoDate> IsoDate::create_for_unix_epoch() {
  auto result = capi::ICU4XIsoDate_create_for_unix_epoch();
  return std::unique_ptr<IsoDate>(IsoDate::FromFFI(result));
}

inline std::unique_ptr<Date> IsoDate::to_calendar(const Calendar& calendar) const {
  auto result = capi::ICU4XIsoDate_to_calendar(this->AsFFI(),
    calendar.AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline std::unique_ptr<Date> IsoDate::to_any() const {
  auto result = capi::ICU4XIsoDate_to_any(this->AsFFI());
  return std::unique_ptr<Date>(Date::FromFFI(result));
}

inline uint16_t IsoDate::day_of_year() const {
  auto result = capi::ICU4XIsoDate_day_of_year(this->AsFFI());
  return result;
}

inline uint32_t IsoDate::day_of_month() const {
  auto result = capi::ICU4XIsoDate_day_of_month(this->AsFFI());
  return result;
}

inline IsoWeekday IsoDate::day_of_week() const {
  auto result = capi::ICU4XIsoDate_day_of_week(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint32_t IsoDate::week_of_month(IsoWeekday first_weekday) const {
  auto result = capi::ICU4XIsoDate_week_of_month(this->AsFFI(),
    first_weekday.AsFFI());
  return result;
}

inline WeekOf IsoDate::week_of_year(const WeekCalculator& calculator) const {
  auto result = capi::ICU4XIsoDate_week_of_year(this->AsFFI(),
    calculator.AsFFI());
  return WeekOf::FromFFI(result);
}

inline uint32_t IsoDate::month() const {
  auto result = capi::ICU4XIsoDate_month(this->AsFFI());
  return result;
}

inline int32_t IsoDate::year() const {
  auto result = capi::ICU4XIsoDate_year(this->AsFFI());
  return result;
}

inline bool IsoDate::is_in_leap_year() const {
  auto result = capi::ICU4XIsoDate_is_in_leap_year(this->AsFFI());
  return result;
}

inline uint8_t IsoDate::months_in_year() const {
  auto result = capi::ICU4XIsoDate_months_in_year(this->AsFFI());
  return result;
}

inline uint8_t IsoDate::days_in_month() const {
  auto result = capi::ICU4XIsoDate_days_in_month(this->AsFFI());
  return result;
}

inline uint16_t IsoDate::days_in_year() const {
  auto result = capi::ICU4XIsoDate_days_in_year(this->AsFFI());
  return result;
}

inline const capi::IsoDate* IsoDate::AsFFI() const {
  return reinterpret_cast<const capi::IsoDate*>(this);
}

inline capi::IsoDate* IsoDate::AsFFI() {
  return reinterpret_cast<capi::IsoDate*>(this);
}

inline const IsoDate* IsoDate::FromFFI(const capi::IsoDate* ptr) {
  return reinterpret_cast<const IsoDate*>(ptr);
}

inline IsoDate* IsoDate::FromFFI(capi::IsoDate* ptr) {
  return reinterpret_cast<IsoDate*>(ptr);
}

inline void IsoDate::operator delete(void* ptr) {
  capi::ICU4XIsoDate_destroy(reinterpret_cast<capi::IsoDate*>(ptr));
}


#endif // IsoDate_HPP

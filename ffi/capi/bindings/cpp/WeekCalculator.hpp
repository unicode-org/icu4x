#ifndef WeekCalculator_HPP
#define WeekCalculator_HPP

#include "WeekCalculator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "IsoWeekday.hpp"
#include "Locale.hpp"
#include "WeekendContainsDay.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_WeekCalculator_create_mv1_result {union {diplomat::capi::WeekCalculator* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_WeekCalculator_create_mv1_result;
    icu4x_WeekCalculator_create_mv1_result icu4x_WeekCalculator_create_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    diplomat::capi::WeekCalculator* icu4x_WeekCalculator_create_from_first_day_of_week_and_min_week_days_mv1(diplomat::capi::IsoWeekday first_weekday, uint8_t min_week_days);
    
    diplomat::capi::IsoWeekday icu4x_WeekCalculator_first_weekday_mv1(const diplomat::capi::WeekCalculator* self);
    
    uint8_t icu4x_WeekCalculator_min_week_days_mv1(const diplomat::capi::WeekCalculator* self);
    
    diplomat::capi::WeekendContainsDay icu4x_WeekCalculator_weekend_mv1(const diplomat::capi::WeekCalculator* self);
    
    
    void icu4x_WeekCalculator_destroy_mv1(WeekCalculator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<WeekCalculator>, DataError> WeekCalculator::create(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::icu4x_WeekCalculator_create_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<WeekCalculator>, DataError>(diplomat::Ok<std::unique_ptr<WeekCalculator>>(std::unique_ptr<WeekCalculator>(WeekCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<WeekCalculator>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<WeekCalculator> WeekCalculator::create_from_first_day_of_week_and_min_week_days(IsoWeekday first_weekday, uint8_t min_week_days) {
  auto result = diplomat::capi::icu4x_WeekCalculator_create_from_first_day_of_week_and_min_week_days_mv1(first_weekday.AsFFI(),
    min_week_days);
  return std::unique_ptr<WeekCalculator>(WeekCalculator::FromFFI(result));
}

inline IsoWeekday WeekCalculator::first_weekday() const {
  auto result = diplomat::capi::icu4x_WeekCalculator_first_weekday_mv1(this->AsFFI());
  return IsoWeekday::FromFFI(result);
}

inline uint8_t WeekCalculator::min_week_days() const {
  auto result = diplomat::capi::icu4x_WeekCalculator_min_week_days_mv1(this->AsFFI());
  return result;
}

inline WeekendContainsDay WeekCalculator::weekend() const {
  auto result = diplomat::capi::icu4x_WeekCalculator_weekend_mv1(this->AsFFI());
  return WeekendContainsDay::FromFFI(result);
}

inline const diplomat::capi::WeekCalculator* WeekCalculator::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::WeekCalculator*>(this);
}

inline diplomat::capi::WeekCalculator* WeekCalculator::AsFFI() {
  return reinterpret_cast<diplomat::capi::WeekCalculator*>(this);
}

inline const WeekCalculator* WeekCalculator::FromFFI(const diplomat::capi::WeekCalculator* ptr) {
  return reinterpret_cast<const WeekCalculator*>(ptr);
}

inline WeekCalculator* WeekCalculator::FromFFI(diplomat::capi::WeekCalculator* ptr) {
  return reinterpret_cast<WeekCalculator*>(ptr);
}

inline void WeekCalculator::operator delete(void* ptr) {
  diplomat::capi::icu4x_WeekCalculator_destroy_mv1(reinterpret_cast<diplomat::capi::WeekCalculator*>(ptr));
}


#endif // WeekCalculator_HPP

#ifndef ICU4XWeekCalculator_HPP
#define ICU4XWeekCalculator_HPP

#include "ICU4XWeekCalculator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIsoWeekday.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XWeekCalculator.h"
#include "ICU4XWeekendContainsDay.hpp"


inline diplomat::result<std::unique_ptr<ICU4XWeekCalculator>, ICU4XError> ICU4XWeekCalculator::create(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XWeekCalculator_create(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XWeekCalculator>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XWeekCalculator>>(std::unique_ptr<ICU4XWeekCalculator>(ICU4XWeekCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XWeekCalculator>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XWeekCalculator> ICU4XWeekCalculator::create_from_first_day_of_week_and_min_week_days(ICU4XIsoWeekday first_weekday, uint8_t min_week_days) {
  auto result = capi::ICU4XWeekCalculator_create_from_first_day_of_week_and_min_week_days(first_weekday.AsFFI(),
    min_week_days);
  return std::unique_ptr<ICU4XWeekCalculator>(ICU4XWeekCalculator::FromFFI(result));
}

inline ICU4XIsoWeekday ICU4XWeekCalculator::first_weekday() const {
  auto result = capi::ICU4XWeekCalculator_first_weekday(this->AsFFI());
  return ICU4XIsoWeekday::FromFFI(result);
}

inline uint8_t ICU4XWeekCalculator::min_week_days() const {
  auto result = capi::ICU4XWeekCalculator_min_week_days(this->AsFFI());
  return result;
}

inline ICU4XWeekendContainsDay ICU4XWeekCalculator::weekend() const {
  auto result = capi::ICU4XWeekCalculator_weekend(this->AsFFI());
  return ICU4XWeekendContainsDay::FromFFI(result);
}

inline const capi::ICU4XWeekCalculator* ICU4XWeekCalculator::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XWeekCalculator*>(this);
}

inline capi::ICU4XWeekCalculator* ICU4XWeekCalculator::AsFFI() {
  return reinterpret_cast<capi::ICU4XWeekCalculator*>(this);
}

inline const ICU4XWeekCalculator* ICU4XWeekCalculator::FromFFI(const capi::ICU4XWeekCalculator* ptr) {
  return reinterpret_cast<const ICU4XWeekCalculator*>(ptr);
}

inline ICU4XWeekCalculator* ICU4XWeekCalculator::FromFFI(capi::ICU4XWeekCalculator* ptr) {
  return reinterpret_cast<ICU4XWeekCalculator*>(ptr);
}

inline void ICU4XWeekCalculator::operator delete(void* ptr) {
  capi::ICU4XWeekCalculator_destroy(reinterpret_cast<capi::ICU4XWeekCalculator*>(ptr));
}


#endif // ICU4XWeekCalculator_HPP

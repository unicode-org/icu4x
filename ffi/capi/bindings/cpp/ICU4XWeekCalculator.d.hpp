#ifndef ICU4XWeekCalculator_D_HPP
#define ICU4XWeekCalculator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XIsoWeekday.d.hpp"
#include "ICU4XWeekCalculator.d.h"
#include "ICU4XWeekendContainsDay.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
struct ICU4XWeekendContainsDay;
class ICU4XError;
class ICU4XIsoWeekday;


class ICU4XWeekCalculator {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XWeekCalculator>, ICU4XError> create(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static std::unique_ptr<ICU4XWeekCalculator> create_from_first_day_of_week_and_min_week_days(ICU4XIsoWeekday first_weekday, uint8_t min_week_days);

  inline ICU4XIsoWeekday first_weekday() const;

  inline uint8_t min_week_days() const;

  inline ICU4XWeekendContainsDay weekend() const;

  inline const capi::ICU4XWeekCalculator* AsFFI() const;
  inline capi::ICU4XWeekCalculator* AsFFI();
  inline static const ICU4XWeekCalculator* FromFFI(const capi::ICU4XWeekCalculator* ptr);
  inline static ICU4XWeekCalculator* FromFFI(capi::ICU4XWeekCalculator* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XWeekCalculator() = delete;
  ICU4XWeekCalculator(const ICU4XWeekCalculator&) = delete;
  ICU4XWeekCalculator(ICU4XWeekCalculator&&) noexcept = delete;
  ICU4XWeekCalculator operator=(const ICU4XWeekCalculator&) = delete;
  ICU4XWeekCalculator operator=(ICU4XWeekCalculator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XWeekCalculator_D_HPP

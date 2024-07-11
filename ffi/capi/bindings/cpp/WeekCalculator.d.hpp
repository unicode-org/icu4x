#ifndef WeekCalculator_D_HPP
#define WeekCalculator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"
#include "IsoWeekday.d.hpp"
#include "WeekendContainsDay.d.hpp"

class DataProvider;
class Locale;
struct WeekendContainsDay;
class DataError;
class IsoWeekday;


namespace capi {
    typedef struct WeekCalculator WeekCalculator;
}

class WeekCalculator {
public:

  inline static diplomat::result<std::unique_ptr<WeekCalculator>, DataError> create(const DataProvider& provider, const Locale& locale);

  inline static std::unique_ptr<WeekCalculator> create_from_first_day_of_week_and_min_week_days(IsoWeekday first_weekday, uint8_t min_week_days);

  inline IsoWeekday first_weekday() const;

  inline uint8_t min_week_days() const;

  inline WeekendContainsDay weekend() const;

  inline const capi::WeekCalculator* AsFFI() const;
  inline capi::WeekCalculator* AsFFI();
  inline static const WeekCalculator* FromFFI(const capi::WeekCalculator* ptr);
  inline static WeekCalculator* FromFFI(capi::WeekCalculator* ptr);
  inline static void operator delete(void* ptr);
private:
  WeekCalculator() = delete;
  WeekCalculator(const WeekCalculator&) = delete;
  WeekCalculator(WeekCalculator&&) noexcept = delete;
  WeekCalculator operator=(const WeekCalculator&) = delete;
  WeekCalculator operator=(WeekCalculator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // WeekCalculator_D_HPP

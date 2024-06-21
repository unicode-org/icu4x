#ifndef ICU4XIsoDate_D_HPP
#define ICU4XIsoDate_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendarError.d.hpp"
#include "ICU4XIsoWeekday.d.hpp"
#include "ICU4XWeekOf.d.hpp"

class ICU4XCalendar;
class ICU4XDate;
class ICU4XWeekCalculator;
struct ICU4XWeekOf;
class ICU4XCalendarError;
class ICU4XIsoWeekday;


namespace capi {
    typedef struct ICU4XIsoDate ICU4XIsoDate;
}

class ICU4XIsoDate {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XIsoDate>, ICU4XCalendarError> create(int32_t year, uint8_t month, uint8_t day);

  inline static std::unique_ptr<ICU4XIsoDate> create_for_unix_epoch();

  inline std::unique_ptr<ICU4XDate> to_calendar(const ICU4XCalendar& calendar) const;

  inline std::unique_ptr<ICU4XDate> to_any() const;

  inline uint16_t day_of_year() const;

  inline uint32_t day_of_month() const;

  inline ICU4XIsoWeekday day_of_week() const;

  inline uint32_t week_of_month(ICU4XIsoWeekday first_weekday) const;

  inline ICU4XWeekOf week_of_year(const ICU4XWeekCalculator& calculator) const;

  inline uint32_t month() const;

  inline int32_t year() const;

  inline bool is_in_leap_year() const;

  inline uint8_t months_in_year() const;

  inline uint8_t days_in_month() const;

  inline uint16_t days_in_year() const;

  inline const capi::ICU4XIsoDate* AsFFI() const;
  inline capi::ICU4XIsoDate* AsFFI();
  inline static const ICU4XIsoDate* FromFFI(const capi::ICU4XIsoDate* ptr);
  inline static ICU4XIsoDate* FromFFI(capi::ICU4XIsoDate* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XIsoDate() = delete;
  ICU4XIsoDate(const ICU4XIsoDate&) = delete;
  ICU4XIsoDate(ICU4XIsoDate&&) noexcept = delete;
  ICU4XIsoDate operator=(const ICU4XIsoDate&) = delete;
  ICU4XIsoDate operator=(ICU4XIsoDate&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XIsoDate_D_HPP

#ifndef ICU4XIsoDateTime_D_HPP
#define ICU4XIsoDateTime_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XIsoWeekday.d.hpp"
#include "ICU4XWeekOf.d.hpp"

class ICU4XCalendar;
class ICU4XDateTime;
class ICU4XIsoDate;
class ICU4XTime;
class ICU4XWeekCalculator;
struct ICU4XWeekOf;
class ICU4XError;
class ICU4XIsoWeekday;


class ICU4XIsoDateTime {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XIsoDateTime>, ICU4XError> create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

  inline static std::unique_ptr<ICU4XIsoDateTime> crate_from_date_and_time(const ICU4XIsoDate& date, const ICU4XTime& time);

  inline static std::unique_ptr<ICU4XIsoDateTime> local_unix_epoch();

  inline static std::unique_ptr<ICU4XIsoDateTime> create_from_minutes_since_local_unix_epoch(int32_t minutes);

  inline std::unique_ptr<ICU4XIsoDate> date() const;

  inline std::unique_ptr<ICU4XTime> time() const;

  inline std::unique_ptr<ICU4XDateTime> to_any() const;

  inline int32_t minutes_since_local_unix_epoch() const;

  inline std::unique_ptr<ICU4XDateTime> to_calendar(const ICU4XCalendar& calendar) const;

  inline uint8_t hour() const;

  inline uint8_t minute() const;

  inline uint8_t second() const;

  inline uint32_t nanosecond() const;

  inline uint16_t day_of_year() const;

  inline uint32_t day_of_month() const;

  inline ICU4XIsoWeekday day_of_week() const;

  inline uint32_t week_of_month(ICU4XIsoWeekday first_weekday) const;

  inline diplomat::result<ICU4XWeekOf, ICU4XError> week_of_year(const ICU4XWeekCalculator& calculator) const;

  inline uint32_t month() const;

  inline int32_t year() const;

  inline bool is_in_leap_year() const;

  inline uint8_t months_in_year() const;

  inline uint8_t days_in_month() const;

  inline uint16_t days_in_year() const;

  inline const capi::ICU4XIsoDateTime* AsFFI() const;
  inline capi::ICU4XIsoDateTime* AsFFI();
  inline static const ICU4XIsoDateTime* FromFFI(const capi::ICU4XIsoDateTime* ptr);
  inline static ICU4XIsoDateTime* FromFFI(capi::ICU4XIsoDateTime* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XIsoDateTime() = delete;
  ICU4XIsoDateTime(const ICU4XIsoDateTime&) = delete;
  ICU4XIsoDateTime(ICU4XIsoDateTime&&) noexcept = delete;
  ICU4XIsoDateTime operator=(const ICU4XIsoDateTime&) = delete;
  ICU4XIsoDateTime operator=(ICU4XIsoDateTime&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XIsoDateTime_D_HPP

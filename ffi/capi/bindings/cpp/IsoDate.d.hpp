#ifndef IsoDate_D_HPP
#define IsoDate_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CalendarError.d.hpp"
#include "IsoWeekday.d.hpp"
#include "WeekOf.d.hpp"

class Calendar;
class Date;
class WeekCalculator;
struct WeekOf;
class CalendarError;
class IsoWeekday;


namespace capi {
    typedef struct IsoDate IsoDate;
}

class IsoDate {
public:

  inline static diplomat::result<std::unique_ptr<IsoDate>, CalendarError> create(int32_t year, uint8_t month, uint8_t day);

  inline static std::unique_ptr<IsoDate> create_for_unix_epoch();

  inline std::unique_ptr<Date> to_calendar(const Calendar& calendar) const;

  inline std::unique_ptr<Date> to_any() const;

  inline uint16_t day_of_year() const;

  inline uint32_t day_of_month() const;

  inline IsoWeekday day_of_week() const;

  inline uint32_t week_of_month(IsoWeekday first_weekday) const;

  inline WeekOf week_of_year(const WeekCalculator& calculator) const;

  inline uint32_t month() const;

  inline int32_t year() const;

  inline bool is_in_leap_year() const;

  inline uint8_t months_in_year() const;

  inline uint8_t days_in_month() const;

  inline uint16_t days_in_year() const;

  inline const capi::IsoDate* AsFFI() const;
  inline capi::IsoDate* AsFFI();
  inline static const IsoDate* FromFFI(const capi::IsoDate* ptr);
  inline static IsoDate* FromFFI(capi::IsoDate* ptr);
  inline static void operator delete(void* ptr);
private:
  IsoDate() = delete;
  IsoDate(const IsoDate&) = delete;
  IsoDate(IsoDate&&) noexcept = delete;
  IsoDate operator=(const IsoDate&) = delete;
  IsoDate operator=(IsoDate&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // IsoDate_D_HPP

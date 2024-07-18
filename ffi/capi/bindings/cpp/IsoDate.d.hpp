#ifndef IsoDate_D_HPP
#define IsoDate_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Calendar; }
class Calendar;
namespace diplomat::capi { struct Date; }
class Date;
namespace diplomat::capi { struct WeekCalculator; }
class WeekCalculator;
struct WeekOf;
class CalendarError;
class FromIxdtfError;
class IsoWeekday;


namespace diplomat {
namespace capi {
    struct IsoDate;
} // namespace capi
} // namespace

class IsoDate {
public:

  inline static diplomat::result<std::unique_ptr<IsoDate>, CalendarError> create(int32_t year, uint8_t month, uint8_t day);

  inline static std::unique_ptr<IsoDate> unix_epoch();

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

  inline const diplomat::capi::IsoDate* AsFFI() const;
  inline diplomat::capi::IsoDate* AsFFI();
  inline static const IsoDate* FromFFI(const diplomat::capi::IsoDate* ptr);
  inline static IsoDate* FromFFI(diplomat::capi::IsoDate* ptr);
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

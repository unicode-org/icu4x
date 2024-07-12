#ifndef IsoDateTime_D_HPP
#define IsoDateTime_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Calendar; }
class Calendar;
namespace diplomat::capi { struct DateTime; }
class DateTime;
namespace diplomat::capi { struct IsoDate; }
class IsoDate;
namespace diplomat::capi { struct Time; }
class Time;
namespace diplomat::capi { struct WeekCalculator; }
class WeekCalculator;
struct WeekOf;
class CalendarError;
class IsoWeekday;


namespace diplomat {
namespace capi {
    struct IsoDateTime;
} // namespace capi
} // namespace

class IsoDateTime {
public:

  inline static diplomat::result<std::unique_ptr<IsoDateTime>, CalendarError> create(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

  inline static std::unique_ptr<IsoDateTime> crate_from_date_and_time(const IsoDate& date, const Time& time);

  inline static std::unique_ptr<IsoDateTime> local_unix_epoch();

  inline static std::unique_ptr<IsoDateTime> create_from_minutes_since_local_unix_epoch(int32_t minutes);

  inline std::unique_ptr<IsoDate> date() const;

  inline std::unique_ptr<Time> time() const;

  inline std::unique_ptr<DateTime> to_any() const;

  inline int32_t minutes_since_local_unix_epoch() const;

  inline std::unique_ptr<DateTime> to_calendar(const Calendar& calendar) const;

  inline uint8_t hour() const;

  inline uint8_t minute() const;

  inline uint8_t second() const;

  inline uint32_t nanosecond() const;

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

  inline const diplomat::capi::IsoDateTime* AsFFI() const;
  inline diplomat::capi::IsoDateTime* AsFFI();
  inline static const IsoDateTime* FromFFI(const diplomat::capi::IsoDateTime* ptr);
  inline static IsoDateTime* FromFFI(diplomat::capi::IsoDateTime* ptr);
  inline static void operator delete(void* ptr);
private:
  IsoDateTime() = delete;
  IsoDateTime(const IsoDateTime&) = delete;
  IsoDateTime(IsoDateTime&&) noexcept = delete;
  IsoDateTime operator=(const IsoDateTime&) = delete;
  IsoDateTime operator=(IsoDateTime&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // IsoDateTime_D_HPP

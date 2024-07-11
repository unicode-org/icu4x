#ifndef Date_D_HPP
#define Date_D_HPP

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
class IsoDate;
class WeekCalculator;
struct WeekOf;
class CalendarError;
class IsoWeekday;


namespace capi {
    typedef struct Date Date;
}

class Date {
public:

  inline static diplomat::result<std::unique_ptr<Date>, CalendarError> create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, const Calendar& calendar);

  inline static diplomat::result<std::unique_ptr<Date>, CalendarError> create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, const Calendar& calendar);

  inline std::unique_ptr<Date> to_calendar(const Calendar& calendar) const;

  inline std::unique_ptr<IsoDate> to_iso() const;

  inline uint16_t day_of_year() const;

  inline uint32_t day_of_month() const;

  inline IsoWeekday day_of_week() const;

  inline uint32_t week_of_month(IsoWeekday first_weekday) const;

  inline WeekOf week_of_year(const WeekCalculator& calculator) const;

  inline uint32_t ordinal_month() const;

  inline std::string month_code() const;

  inline int32_t year_in_era() const;

  inline std::string era() const;

  inline uint8_t months_in_year() const;

  inline uint8_t days_in_month() const;

  inline uint16_t days_in_year() const;

  inline std::unique_ptr<Calendar> calendar() const;

  inline const capi::Date* AsFFI() const;
  inline capi::Date* AsFFI();
  inline static const Date* FromFFI(const capi::Date* ptr);
  inline static Date* FromFFI(capi::Date* ptr);
  inline static void operator delete(void* ptr);
private:
  Date() = delete;
  Date(const Date&) = delete;
  Date(Date&&) noexcept = delete;
  Date operator=(const Date&) = delete;
  Date operator=(Date&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Date_D_HPP

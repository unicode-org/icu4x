#ifndef ICU4XDateTime_D_HPP
#define ICU4XDateTime_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDateTime.d.h"
#include "ICU4XError.d.hpp"
#include "ICU4XIsoWeekday.d.hpp"
#include "ICU4XWeekOf.d.hpp"

class ICU4XCalendar;
class ICU4XDate;
class ICU4XIsoDateTime;
class ICU4XTime;
class ICU4XWeekCalculator;
struct ICU4XWeekOf;
class ICU4XError;
class ICU4XIsoWeekday;


class ICU4XDateTime {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XError> create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar);

  inline static diplomat::result<std::unique_ptr<ICU4XDateTime>, ICU4XError> create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar);

  inline static std::unique_ptr<ICU4XDateTime> create_from_date_and_time(const ICU4XDate& date, const ICU4XTime& time);

  inline std::unique_ptr<ICU4XDate> date() const;

  inline std::unique_ptr<ICU4XTime> time() const;

  inline std::unique_ptr<ICU4XIsoDateTime> to_iso() const;

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

  inline uint32_t ordinal_month() const;

  inline std::string month_code() const;

  inline int32_t year_in_era() const;

  inline std::string era() const;

  inline uint8_t months_in_year() const;

  inline uint8_t days_in_month() const;

  inline uint16_t days_in_year() const;

  inline std::unique_ptr<ICU4XCalendar> calendar() const;

  inline const capi::ICU4XDateTime* AsFFI() const;
  inline capi::ICU4XDateTime* AsFFI();
  inline static const ICU4XDateTime* FromFFI(const capi::ICU4XDateTime* ptr);
  inline static ICU4XDateTime* FromFFI(capi::ICU4XDateTime* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDateTime() = delete;
  ICU4XDateTime(const ICU4XDateTime&) = delete;
  ICU4XDateTime(ICU4XDateTime&&) noexcept = delete;
  ICU4XDateTime operator=(const ICU4XDateTime&) = delete;
  ICU4XDateTime operator=(ICU4XDateTime&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XDateTime_D_HPP

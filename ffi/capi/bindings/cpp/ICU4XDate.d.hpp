#ifndef ICU4XDate_D_HPP
#define ICU4XDate_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendarError.d.hpp"
#include "ICU4XDate.d.h"
#include "ICU4XIsoWeekday.d.hpp"
#include "ICU4XWeekOf.d.hpp"

class ICU4XCalendar;
class ICU4XIsoDate;
class ICU4XWeekCalculator;
struct ICU4XWeekOf;
class ICU4XCalendarError;
class ICU4XIsoWeekday;


class ICU4XDate {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError> create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, const ICU4XCalendar& calendar);

  inline static diplomat::result<std::unique_ptr<ICU4XDate>, ICU4XCalendarError> create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, const ICU4XCalendar& calendar);

  inline std::unique_ptr<ICU4XDate> to_calendar(const ICU4XCalendar& calendar) const;

  inline std::unique_ptr<ICU4XIsoDate> to_iso() const;

  inline uint16_t day_of_year() const;

  inline uint32_t day_of_month() const;

  inline ICU4XIsoWeekday day_of_week() const;

  inline uint32_t week_of_month(ICU4XIsoWeekday first_weekday) const;

  inline ICU4XWeekOf week_of_year(const ICU4XWeekCalculator& calculator) const;

  inline uint32_t ordinal_month() const;

  inline std::string month_code() const;

  inline int32_t year_in_era() const;

  inline std::string era() const;

  inline uint8_t months_in_year() const;

  inline uint8_t days_in_month() const;

  inline uint16_t days_in_year() const;

  inline std::unique_ptr<ICU4XCalendar> calendar() const;

  inline const capi::ICU4XDate* AsFFI() const;
  inline capi::ICU4XDate* AsFFI();
  inline static const ICU4XDate* FromFFI(const capi::ICU4XDate* ptr);
  inline static ICU4XDate* FromFFI(capi::ICU4XDate* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDate() = delete;
  ICU4XDate(const ICU4XDate&) = delete;
  ICU4XDate(ICU4XDate&&) noexcept = delete;
  ICU4XDate operator=(const ICU4XDate&) = delete;
  ICU4XDate operator=(ICU4XDate&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XDate_D_HPP

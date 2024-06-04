#ifndef ICU4XZonedDateTimeFormatter_D_HPP
#define ICU4XZonedDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDateLength.d.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XIsoTimeZoneOptions.d.hpp"
#include "ICU4XTimeLength.d.hpp"
#include "ICU4XZonedDateTimeFormatter.d.h"

class ICU4XCustomTimeZone;
class ICU4XDataProvider;
class ICU4XDateTime;
class ICU4XIsoDateTime;
class ICU4XLocale;
struct ICU4XIsoTimeZoneOptions;
class ICU4XDateLength;
class ICU4XError;
class ICU4XTimeLength;


class ICU4XZonedDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError> create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

  inline static diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError> create_with_lengths_and_iso_8601_time_zone_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options);

  inline diplomat::result<std::string, ICU4XError> format_datetime_with_custom_time_zone(const ICU4XDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const;

  inline diplomat::result<std::string, ICU4XError> format_iso_datetime_with_custom_time_zone(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const;

  inline const capi::ICU4XZonedDateTimeFormatter* AsFFI() const;
  inline capi::ICU4XZonedDateTimeFormatter* AsFFI();
  inline static const ICU4XZonedDateTimeFormatter* FromFFI(const capi::ICU4XZonedDateTimeFormatter* ptr);
  inline static ICU4XZonedDateTimeFormatter* FromFFI(capi::ICU4XZonedDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XZonedDateTimeFormatter() = delete;
  ICU4XZonedDateTimeFormatter(const ICU4XZonedDateTimeFormatter&) = delete;
  ICU4XZonedDateTimeFormatter(ICU4XZonedDateTimeFormatter&&) noexcept = delete;
  ICU4XZonedDateTimeFormatter operator=(const ICU4XZonedDateTimeFormatter&) = delete;
  ICU4XZonedDateTimeFormatter operator=(ICU4XZonedDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XZonedDateTimeFormatter_D_HPP

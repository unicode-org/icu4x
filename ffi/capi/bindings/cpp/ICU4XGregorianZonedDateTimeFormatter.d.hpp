#ifndef ICU4XGregorianZonedDateTimeFormatter_D_HPP
#define ICU4XGregorianZonedDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDateLength.d.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XGregorianZonedDateTimeFormatter.d.h"
#include "ICU4XIsoTimeZoneOptions.d.hpp"
#include "ICU4XTimeLength.d.hpp"

class ICU4XCustomTimeZone;
class ICU4XDataProvider;
class ICU4XIsoDateTime;
class ICU4XLocale;
struct ICU4XIsoTimeZoneOptions;
class ICU4XDateLength;
class ICU4XError;
class ICU4XTimeLength;


class ICU4XGregorianZonedDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError> create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

  inline static diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError> create_with_lengths_and_iso_8601_time_zone_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options);

  inline std::string format_iso_datetime_with_custom_time_zone(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const;

  inline const capi::ICU4XGregorianZonedDateTimeFormatter* AsFFI() const;
  inline capi::ICU4XGregorianZonedDateTimeFormatter* AsFFI();
  inline static const ICU4XGregorianZonedDateTimeFormatter* FromFFI(const capi::ICU4XGregorianZonedDateTimeFormatter* ptr);
  inline static ICU4XGregorianZonedDateTimeFormatter* FromFFI(capi::ICU4XGregorianZonedDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGregorianZonedDateTimeFormatter() = delete;
  ICU4XGregorianZonedDateTimeFormatter(const ICU4XGregorianZonedDateTimeFormatter&) = delete;
  ICU4XGregorianZonedDateTimeFormatter(ICU4XGregorianZonedDateTimeFormatter&&) noexcept = delete;
  ICU4XGregorianZonedDateTimeFormatter operator=(const ICU4XGregorianZonedDateTimeFormatter&) = delete;
  ICU4XGregorianZonedDateTimeFormatter operator=(ICU4XGregorianZonedDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGregorianZonedDateTimeFormatter_D_HPP

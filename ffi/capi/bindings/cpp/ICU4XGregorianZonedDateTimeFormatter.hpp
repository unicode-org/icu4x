#ifndef ICU4XGregorianZonedDateTimeFormatter_HPP
#define ICU4XGregorianZonedDateTimeFormatter_HPP

#include "ICU4XGregorianZonedDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCustomTimeZone.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDateLength.hpp"
#include "ICU4XError.hpp"
#include "ICU4XGregorianZonedDateTimeFormatter.h"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XIsoTimeZoneOptions.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XTimeLength.hpp"


inline diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError> ICU4XGregorianZonedDateTimeFormatter::create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length) {
  auto result = capi::ICU4XGregorianZonedDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>>(std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>(ICU4XGregorianZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError> ICU4XGregorianZonedDateTimeFormatter::create_with_lengths_and_iso_8601_time_zone_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options) {
  auto result = capi::ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI(),
    zone_options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>>(std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>(ICU4XGregorianZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::string ICU4XGregorianZonedDateTimeFormatter::format_iso_datetime_with_custom_time_zone(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return output;
}

inline const capi::ICU4XGregorianZonedDateTimeFormatter* ICU4XGregorianZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGregorianZonedDateTimeFormatter*>(this);
}

inline capi::ICU4XGregorianZonedDateTimeFormatter* ICU4XGregorianZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XGregorianZonedDateTimeFormatter*>(this);
}

inline const ICU4XGregorianZonedDateTimeFormatter* ICU4XGregorianZonedDateTimeFormatter::FromFFI(const capi::ICU4XGregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const ICU4XGregorianZonedDateTimeFormatter*>(ptr);
}

inline ICU4XGregorianZonedDateTimeFormatter* ICU4XGregorianZonedDateTimeFormatter::FromFFI(capi::ICU4XGregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<ICU4XGregorianZonedDateTimeFormatter*>(ptr);
}

inline void ICU4XGregorianZonedDateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XGregorianZonedDateTimeFormatter_destroy(reinterpret_cast<capi::ICU4XGregorianZonedDateTimeFormatter*>(ptr));
}


#endif // ICU4XGregorianZonedDateTimeFormatter_HPP

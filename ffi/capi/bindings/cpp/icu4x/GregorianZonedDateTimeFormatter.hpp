#ifndef icu4x_GregorianZonedDateTimeFormatter_HPP
#define icu4x_GregorianZonedDateTimeFormatter_HPP

#include "GregorianZonedDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "DateTimeLength.hpp"
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"
#include "TimeZone.hpp"
#include "TimeZoneCalculator.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_GregorianZonedDateTimeFormatter_create_with_length_mv1_result {union {icu4x::capi::GregorianZonedDateTimeFormatter* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_GregorianZonedDateTimeFormatter_create_with_length_mv1_result;
    icu4x_GregorianZonedDateTimeFormatter_create_with_length_mv1_result icu4x_GregorianZonedDateTimeFormatter_create_with_length_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength length);
    
    void icu4x_GregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_mv1(const icu4x::capi::GregorianZonedDateTimeFormatter* self, const icu4x::capi::IsoDateTime* datetime, const icu4x::capi::TimeZone* time_zone, const icu4x::capi::TimeZoneCalculator* timezone_calculator, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_GregorianZonedDateTimeFormatter_destroy_mv1(GregorianZonedDateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::GregorianZonedDateTimeFormatter>, icu4x::Error> icu4x::GregorianZonedDateTimeFormatter::create_with_length(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DateTimeLength length) {
  auto result = icu4x::capi::icu4x_GregorianZonedDateTimeFormatter_create_with_length_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::GregorianZonedDateTimeFormatter>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::GregorianZonedDateTimeFormatter>>(std::unique_ptr<icu4x::GregorianZonedDateTimeFormatter>(icu4x::GregorianZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::GregorianZonedDateTimeFormatter>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline std::string icu4x::GregorianZonedDateTimeFormatter::format_iso_datetime_with_custom_time_zone(const icu4x::IsoDateTime& datetime, const icu4x::TimeZone& time_zone, const icu4x::TimeZoneCalculator& timezone_calculator) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_GregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_mv1(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    timezone_calculator.AsFFI(),
    &write);
  return output;
}

inline const icu4x::capi::GregorianZonedDateTimeFormatter* icu4x::GregorianZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::GregorianZonedDateTimeFormatter*>(this);
}

inline icu4x::capi::GregorianZonedDateTimeFormatter* icu4x::GregorianZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::GregorianZonedDateTimeFormatter*>(this);
}

inline const icu4x::GregorianZonedDateTimeFormatter* icu4x::GregorianZonedDateTimeFormatter::FromFFI(const icu4x::capi::GregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const icu4x::GregorianZonedDateTimeFormatter*>(ptr);
}

inline icu4x::GregorianZonedDateTimeFormatter* icu4x::GregorianZonedDateTimeFormatter::FromFFI(icu4x::capi::GregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<icu4x::GregorianZonedDateTimeFormatter*>(ptr);
}

inline void icu4x::GregorianZonedDateTimeFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_GregorianZonedDateTimeFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::GregorianZonedDateTimeFormatter*>(ptr));
}


#endif // icu4x_GregorianZonedDateTimeFormatter_HPP

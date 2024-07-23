#ifndef GregorianDateFormatter_HPP
#define GregorianDateFormatter_HPP

#include "GregorianDateFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataProvider.hpp"
#include "DateLength.hpp"
#include "Error.hpp"
#include "IsoDate.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_GregorianDateFormatter_create_with_length_mv1_result {union {diplomat::capi::GregorianDateFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} icu4x_GregorianDateFormatter_create_with_length_mv1_result;
    icu4x_GregorianDateFormatter_create_with_length_mv1_result icu4x_GregorianDateFormatter_create_with_length_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DateLength length);
    
    void icu4x_GregorianDateFormatter_format_iso_date_mv1(const diplomat::capi::GregorianDateFormatter* self, const diplomat::capi::IsoDate* value, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_GregorianDateFormatter_format_iso_datetime_mv1(const diplomat::capi::GregorianDateFormatter* self, const diplomat::capi::IsoDateTime* value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_GregorianDateFormatter_destroy_mv1(GregorianDateFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<GregorianDateFormatter>, Error> GregorianDateFormatter::create_with_length(const DataProvider& provider, const Locale& locale, DateLength length) {
  auto result = diplomat::capi::icu4x_GregorianDateFormatter_create_with_length_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GregorianDateFormatter>, Error>(diplomat::Ok<std::unique_ptr<GregorianDateFormatter>>(std::unique_ptr<GregorianDateFormatter>(GregorianDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GregorianDateFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline std::string GregorianDateFormatter::format_iso_date(const IsoDate& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_GregorianDateFormatter_format_iso_date_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline std::string GregorianDateFormatter::format_iso_datetime(const IsoDateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_GregorianDateFormatter_format_iso_datetime_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::GregorianDateFormatter* GregorianDateFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::GregorianDateFormatter*>(this);
}

inline diplomat::capi::GregorianDateFormatter* GregorianDateFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::GregorianDateFormatter*>(this);
}

inline const GregorianDateFormatter* GregorianDateFormatter::FromFFI(const diplomat::capi::GregorianDateFormatter* ptr) {
  return reinterpret_cast<const GregorianDateFormatter*>(ptr);
}

inline GregorianDateFormatter* GregorianDateFormatter::FromFFI(diplomat::capi::GregorianDateFormatter* ptr) {
  return reinterpret_cast<GregorianDateFormatter*>(ptr);
}

inline void GregorianDateFormatter::operator delete(void* ptr) {
  diplomat::capi::icu4x_GregorianDateFormatter_destroy_mv1(reinterpret_cast<diplomat::capi::GregorianDateFormatter*>(ptr));
}


#endif // GregorianDateFormatter_HPP

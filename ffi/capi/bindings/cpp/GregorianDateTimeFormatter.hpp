#ifndef GregorianDateTimeFormatter_HPP
#define GregorianDateTimeFormatter_HPP

#include "GregorianDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "DateLength.hpp"
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"
#include "TimeLength.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XGregorianDateTimeFormatter_create_with_lengths_result {union {GregorianDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XGregorianDateTimeFormatter_create_with_lengths_result;
    ICU4XGregorianDateTimeFormatter_create_with_lengths_result ICU4XGregorianDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);
    
    void ICU4XGregorianDateTimeFormatter_format_iso_datetime(const GregorianDateTimeFormatter* self, const IsoDateTime* value, DiplomatWrite* write);
    
    
    void ICU4XGregorianDateTimeFormatter_destroy(GregorianDateTimeFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error> GregorianDateTimeFormatter::create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length) {
  auto result = capi::ICU4XGregorianDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<GregorianDateTimeFormatter>>(std::unique_ptr<GregorianDateTimeFormatter>(GregorianDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline std::string GregorianDateTimeFormatter::format_iso_datetime(const IsoDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XGregorianDateTimeFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const capi::GregorianDateTimeFormatter* GregorianDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::GregorianDateTimeFormatter*>(this);
}

inline capi::GregorianDateTimeFormatter* GregorianDateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::GregorianDateTimeFormatter*>(this);
}

inline const GregorianDateTimeFormatter* GregorianDateTimeFormatter::FromFFI(const capi::GregorianDateTimeFormatter* ptr) {
  return reinterpret_cast<const GregorianDateTimeFormatter*>(ptr);
}

inline GregorianDateTimeFormatter* GregorianDateTimeFormatter::FromFFI(capi::GregorianDateTimeFormatter* ptr) {
  return reinterpret_cast<GregorianDateTimeFormatter*>(ptr);
}

inline void GregorianDateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XGregorianDateTimeFormatter_destroy(reinterpret_cast<capi::GregorianDateTimeFormatter*>(ptr));
}


#endif // GregorianDateTimeFormatter_HPP

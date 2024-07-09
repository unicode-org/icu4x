#ifndef ICU4XGregorianDateTimeFormatter_HPP
#define ICU4XGregorianDateTimeFormatter_HPP

#include "ICU4XGregorianDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDateLength.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XTimeLength.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XGregorianDateTimeFormatter_create_with_lengths_result {union {ICU4XGregorianDateTimeFormatter* ok; ICU4XError err;}; bool is_ok;} ICU4XGregorianDateTimeFormatter_create_with_lengths_result;
    ICU4XGregorianDateTimeFormatter_create_with_lengths_result ICU4XGregorianDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);
    
    void ICU4XGregorianDateTimeFormatter_format_iso_datetime(const ICU4XGregorianDateTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);
    
    
    void ICU4XGregorianDateTimeFormatter_destroy(ICU4XGregorianDateTimeFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XGregorianDateTimeFormatter>, ICU4XError> ICU4XGregorianDateTimeFormatter::create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length) {
  auto result = capi::ICU4XGregorianDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XGregorianDateTimeFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XGregorianDateTimeFormatter>>(std::unique_ptr<ICU4XGregorianDateTimeFormatter>(ICU4XGregorianDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XGregorianDateTimeFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::string ICU4XGregorianDateTimeFormatter::format_iso_datetime(const ICU4XIsoDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XGregorianDateTimeFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const capi::ICU4XGregorianDateTimeFormatter* ICU4XGregorianDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGregorianDateTimeFormatter*>(this);
}

inline capi::ICU4XGregorianDateTimeFormatter* ICU4XGregorianDateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XGregorianDateTimeFormatter*>(this);
}

inline const ICU4XGregorianDateTimeFormatter* ICU4XGregorianDateTimeFormatter::FromFFI(const capi::ICU4XGregorianDateTimeFormatter* ptr) {
  return reinterpret_cast<const ICU4XGregorianDateTimeFormatter*>(ptr);
}

inline ICU4XGregorianDateTimeFormatter* ICU4XGregorianDateTimeFormatter::FromFFI(capi::ICU4XGregorianDateTimeFormatter* ptr) {
  return reinterpret_cast<ICU4XGregorianDateTimeFormatter*>(ptr);
}

inline void ICU4XGregorianDateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XGregorianDateTimeFormatter_destroy(reinterpret_cast<capi::ICU4XGregorianDateTimeFormatter*>(ptr));
}


#endif // ICU4XGregorianDateTimeFormatter_HPP

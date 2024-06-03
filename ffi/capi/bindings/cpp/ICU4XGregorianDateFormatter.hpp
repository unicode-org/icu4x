#ifndef ICU4XGregorianDateFormatter_HPP
#define ICU4XGregorianDateFormatter_HPP

#include "ICU4XGregorianDateFormatter.d.hpp"

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
#include "ICU4XGregorianDateFormatter.h"
#include "ICU4XIsoDate.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XLocale.hpp"


inline diplomat::result<std::unique_ptr<ICU4XGregorianDateFormatter>, ICU4XError> ICU4XGregorianDateFormatter::create_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength length) {
  auto result = capi::ICU4XGregorianDateFormatter_create_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XGregorianDateFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XGregorianDateFormatter>>(std::unique_ptr<ICU4XGregorianDateFormatter>(ICU4XGregorianDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XGregorianDateFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::string ICU4XGregorianDateFormatter::format_iso_date(const ICU4XIsoDate& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XGregorianDateFormatter_format_iso_date(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline std::string ICU4XGregorianDateFormatter::format_iso_datetime(const ICU4XIsoDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XGregorianDateFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const capi::ICU4XGregorianDateFormatter* ICU4XGregorianDateFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGregorianDateFormatter*>(this);
}

inline capi::ICU4XGregorianDateFormatter* ICU4XGregorianDateFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XGregorianDateFormatter*>(this);
}

inline const ICU4XGregorianDateFormatter* ICU4XGregorianDateFormatter::FromFFI(const capi::ICU4XGregorianDateFormatter* ptr) {
  return reinterpret_cast<const ICU4XGregorianDateFormatter*>(ptr);
}

inline ICU4XGregorianDateFormatter* ICU4XGregorianDateFormatter::FromFFI(capi::ICU4XGregorianDateFormatter* ptr) {
  return reinterpret_cast<ICU4XGregorianDateFormatter*>(ptr);
}

inline void ICU4XGregorianDateFormatter::operator delete(void* ptr) {
  capi::ICU4XGregorianDateFormatter_destroy(reinterpret_cast<capi::ICU4XGregorianDateFormatter*>(ptr));
}


#endif // ICU4XGregorianDateFormatter_HPP

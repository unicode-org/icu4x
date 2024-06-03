#ifndef ICU4XCalendar_HPP
#define ICU4XCalendar_HPP

#include "ICU4XCalendar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XAnyCalendarKind.hpp"
#include "ICU4XCalendar.h"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XLocale.hpp"


inline diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XError> ICU4XCalendar::create_for_locale(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XCalendar_create_for_locale(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCalendar>>(std::unique_ptr<ICU4XCalendar>(ICU4XCalendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XError> ICU4XCalendar::create_for_kind(const ICU4XDataProvider& provider, ICU4XAnyCalendarKind kind) {
  auto result = capi::ICU4XCalendar_create_for_kind(provider.AsFFI(),
    kind.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCalendar>>(std::unique_ptr<ICU4XCalendar>(ICU4XCalendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline ICU4XAnyCalendarKind ICU4XCalendar::kind() const {
  auto result = capi::ICU4XCalendar_kind(this->AsFFI());
  return ICU4XAnyCalendarKind::FromFFI(result);
}

inline const capi::ICU4XCalendar* ICU4XCalendar::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCalendar*>(this);
}

inline capi::ICU4XCalendar* ICU4XCalendar::AsFFI() {
  return reinterpret_cast<capi::ICU4XCalendar*>(this);
}

inline const ICU4XCalendar* ICU4XCalendar::FromFFI(const capi::ICU4XCalendar* ptr) {
  return reinterpret_cast<const ICU4XCalendar*>(ptr);
}

inline ICU4XCalendar* ICU4XCalendar::FromFFI(capi::ICU4XCalendar* ptr) {
  return reinterpret_cast<ICU4XCalendar*>(ptr);
}

inline void ICU4XCalendar::operator delete(void* ptr) {
  capi::ICU4XCalendar_destroy(reinterpret_cast<capi::ICU4XCalendar*>(ptr));
}


#endif // ICU4XCalendar_HPP

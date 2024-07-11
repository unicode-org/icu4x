#ifndef Calendar_HPP
#define Calendar_HPP

#include "Calendar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AnyCalendarKind.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCalendar_create_for_locale_result {union {Calendar* ok; DataError err;}; bool is_ok;} ICU4XCalendar_create_for_locale_result;
    ICU4XCalendar_create_for_locale_result ICU4XCalendar_create_for_locale(const DataProvider* provider, const Locale* locale);
    
    typedef struct ICU4XCalendar_create_for_kind_result {union {Calendar* ok; DataError err;}; bool is_ok;} ICU4XCalendar_create_for_kind_result;
    ICU4XCalendar_create_for_kind_result ICU4XCalendar_create_for_kind(const DataProvider* provider, AnyCalendarKind kind);
    
    AnyCalendarKind ICU4XCalendar_kind(const Calendar* self);
    
    
    void ICU4XCalendar_destroy(Calendar* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<Calendar>, DataError> Calendar::create_for_locale(const DataProvider& provider, const Locale& locale) {
  auto result = capi::ICU4XCalendar_create_for_locale(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Ok<std::unique_ptr<Calendar>>(std::unique_ptr<Calendar>(Calendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<Calendar>, DataError> Calendar::create_for_kind(const DataProvider& provider, AnyCalendarKind kind) {
  auto result = capi::ICU4XCalendar_create_for_kind(provider.AsFFI(),
    kind.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Ok<std::unique_ptr<Calendar>>(std::unique_ptr<Calendar>(Calendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Calendar>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline AnyCalendarKind Calendar::kind() const {
  auto result = capi::ICU4XCalendar_kind(this->AsFFI());
  return AnyCalendarKind::FromFFI(result);
}

inline const capi::Calendar* Calendar::AsFFI() const {
  return reinterpret_cast<const capi::Calendar*>(this);
}

inline capi::Calendar* Calendar::AsFFI() {
  return reinterpret_cast<capi::Calendar*>(this);
}

inline const Calendar* Calendar::FromFFI(const capi::Calendar* ptr) {
  return reinterpret_cast<const Calendar*>(ptr);
}

inline Calendar* Calendar::FromFFI(capi::Calendar* ptr) {
  return reinterpret_cast<Calendar*>(ptr);
}

inline void Calendar::operator delete(void* ptr) {
  capi::ICU4XCalendar_destroy(reinterpret_cast<capi::Calendar*>(ptr));
}


#endif // Calendar_HPP

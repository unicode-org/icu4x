#ifndef icu4x_Calendar_HPP
#define icu4x_Calendar_HPP

#include "Calendar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "AnyCalendarKind.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_Calendar_create_for_locale_mv1_result {union {icu4x::capi::Calendar* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_Calendar_create_for_locale_mv1_result;
    icu4x_Calendar_create_for_locale_mv1_result icu4x_Calendar_create_for_locale_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale);
    
    typedef struct icu4x_Calendar_create_for_kind_mv1_result {union {icu4x::capi::Calendar* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_Calendar_create_for_kind_mv1_result;
    icu4x_Calendar_create_for_kind_mv1_result icu4x_Calendar_create_for_kind_mv1(const icu4x::capi::DataProvider* provider, icu4x::capi::AnyCalendarKind kind);
    
    icu4x::capi::AnyCalendarKind icu4x_Calendar_kind_mv1(const icu4x::capi::Calendar* self);
    
    
    void icu4x_Calendar_destroy_mv1(Calendar* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError> icu4x::Calendar::create_for_locale(const icu4x::DataProvider& provider, const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_Calendar_create_for_locale_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::Calendar>>(std::unique_ptr<icu4x::Calendar>(icu4x::Calendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError> icu4x::Calendar::create_for_kind(const icu4x::DataProvider& provider, icu4x::AnyCalendarKind kind) {
  auto result = icu4x::capi::icu4x_Calendar_create_for_kind_mv1(provider.AsFFI(),
    kind.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::Calendar>>(std::unique_ptr<icu4x::Calendar>(icu4x::Calendar::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::AnyCalendarKind icu4x::Calendar::kind() const {
  auto result = icu4x::capi::icu4x_Calendar_kind_mv1(this->AsFFI());
  return icu4x::AnyCalendarKind::FromFFI(result);
}

inline const icu4x::capi::Calendar* icu4x::Calendar::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::Calendar*>(this);
}

inline icu4x::capi::Calendar* icu4x::Calendar::AsFFI() {
  return reinterpret_cast<icu4x::capi::Calendar*>(this);
}

inline const icu4x::Calendar* icu4x::Calendar::FromFFI(const icu4x::capi::Calendar* ptr) {
  return reinterpret_cast<const icu4x::Calendar*>(ptr);
}

inline icu4x::Calendar* icu4x::Calendar::FromFFI(icu4x::capi::Calendar* ptr) {
  return reinterpret_cast<icu4x::Calendar*>(ptr);
}

inline void icu4x::Calendar::operator delete(void* ptr) {
  icu4x::capi::icu4x_Calendar_destroy_mv1(reinterpret_cast<icu4x::capi::Calendar*>(ptr));
}


#endif // icu4x_Calendar_HPP

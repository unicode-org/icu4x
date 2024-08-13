#ifndef icu4x_LocaleDisplayNamesFormatter_HPP
#define icu4x_LocaleDisplayNamesFormatter_HPP

#include "LocaleDisplayNamesFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "DisplayNamesOptionsV1.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_LocaleDisplayNamesFormatter_create_v1_mv1_result {union {icu4x::capi::LocaleDisplayNamesFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_LocaleDisplayNamesFormatter_create_v1_mv1_result;
    icu4x_LocaleDisplayNamesFormatter_create_v1_mv1_result icu4x_LocaleDisplayNamesFormatter_create_v1_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DisplayNamesOptionsV1 options);
    
    void icu4x_LocaleDisplayNamesFormatter_of_mv1(const icu4x::capi::LocaleDisplayNamesFormatter* self, const icu4x::capi::Locale* locale, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_LocaleDisplayNamesFormatter_destroy_mv1(LocaleDisplayNamesFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::LocaleDisplayNamesFormatter>, icu4x::DataError> icu4x::LocaleDisplayNamesFormatter::create_v1(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DisplayNamesOptionsV1 options) {
  auto result = icu4x::capi::icu4x_LocaleDisplayNamesFormatter_create_v1_mv1(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::LocaleDisplayNamesFormatter>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::LocaleDisplayNamesFormatter>>(std::unique_ptr<icu4x::LocaleDisplayNamesFormatter>(icu4x::LocaleDisplayNamesFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::LocaleDisplayNamesFormatter>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::string icu4x::LocaleDisplayNamesFormatter::of(const icu4x::Locale& locale) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_LocaleDisplayNamesFormatter_of_mv1(this->AsFFI(),
    locale.AsFFI(),
    &write);
  return output;
}

inline const icu4x::capi::LocaleDisplayNamesFormatter* icu4x::LocaleDisplayNamesFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::LocaleDisplayNamesFormatter*>(this);
}

inline icu4x::capi::LocaleDisplayNamesFormatter* icu4x::LocaleDisplayNamesFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::LocaleDisplayNamesFormatter*>(this);
}

inline const icu4x::LocaleDisplayNamesFormatter* icu4x::LocaleDisplayNamesFormatter::FromFFI(const icu4x::capi::LocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<const icu4x::LocaleDisplayNamesFormatter*>(ptr);
}

inline icu4x::LocaleDisplayNamesFormatter* icu4x::LocaleDisplayNamesFormatter::FromFFI(icu4x::capi::LocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<icu4x::LocaleDisplayNamesFormatter*>(ptr);
}

inline void icu4x::LocaleDisplayNamesFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_LocaleDisplayNamesFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::LocaleDisplayNamesFormatter*>(ptr));
}


#endif // icu4x_LocaleDisplayNamesFormatter_HPP

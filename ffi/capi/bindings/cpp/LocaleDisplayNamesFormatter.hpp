#ifndef LocaleDisplayNamesFormatter_HPP
#define LocaleDisplayNamesFormatter_HPP

#include "LocaleDisplayNamesFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "DisplayNamesOptionsV1.hpp"
#include "Locale.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleDisplayNamesFormatter_create_result {union {diplomat::capi::LocaleDisplayNamesFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XLocaleDisplayNamesFormatter_create_result;
    ICU4XLocaleDisplayNamesFormatter_create_result ICU4XLocaleDisplayNamesFormatter_create(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DisplayNamesOptionsV1 options);
    
    void ICU4XLocaleDisplayNamesFormatter_of(const diplomat::capi::LocaleDisplayNamesFormatter* self, const diplomat::capi::Locale* locale, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XLocaleDisplayNamesFormatter_destroy(LocaleDisplayNamesFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError> LocaleDisplayNamesFormatter::create(const DataProvider& provider, const Locale& locale, DisplayNamesOptionsV1 options) {
  auto result = diplomat::capi::ICU4XLocaleDisplayNamesFormatter_create(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError>(diplomat::Ok<std::unique_ptr<LocaleDisplayNamesFormatter>>(std::unique_ptr<LocaleDisplayNamesFormatter>(LocaleDisplayNamesFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string LocaleDisplayNamesFormatter::of(const Locale& locale) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XLocaleDisplayNamesFormatter_of(this->AsFFI(),
    locale.AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LocaleDisplayNamesFormatter*>(this);
}

inline diplomat::capi::LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::LocaleDisplayNamesFormatter*>(this);
}

inline const LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::FromFFI(const diplomat::capi::LocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<const LocaleDisplayNamesFormatter*>(ptr);
}

inline LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::FromFFI(diplomat::capi::LocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<LocaleDisplayNamesFormatter*>(ptr);
}

inline void LocaleDisplayNamesFormatter::operator delete(void* ptr) {
  diplomat::capi::ICU4XLocaleDisplayNamesFormatter_destroy(reinterpret_cast<diplomat::capi::LocaleDisplayNamesFormatter*>(ptr));
}


#endif // LocaleDisplayNamesFormatter_HPP

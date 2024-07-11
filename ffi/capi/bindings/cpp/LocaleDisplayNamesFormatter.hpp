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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleDisplayNamesFormatter_create_result {union {LocaleDisplayNamesFormatter* ok; DataError err;}; bool is_ok;} ICU4XLocaleDisplayNamesFormatter_create_result;
    ICU4XLocaleDisplayNamesFormatter_create_result ICU4XLocaleDisplayNamesFormatter_create(const DataProvider* provider, const Locale* locale, DisplayNamesOptionsV1 options);
    
    void ICU4XLocaleDisplayNamesFormatter_of(const LocaleDisplayNamesFormatter* self, const Locale* locale, DiplomatWrite* write);
    
    
    void ICU4XLocaleDisplayNamesFormatter_destroy(LocaleDisplayNamesFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError> LocaleDisplayNamesFormatter::create(const DataProvider& provider, const Locale& locale, DisplayNamesOptionsV1 options) {
  auto result = capi::ICU4XLocaleDisplayNamesFormatter_create(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError>(diplomat::Ok<std::unique_ptr<LocaleDisplayNamesFormatter>>(std::unique_ptr<LocaleDisplayNamesFormatter>(LocaleDisplayNamesFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string LocaleDisplayNamesFormatter::of(const Locale& locale) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocaleDisplayNamesFormatter_of(this->AsFFI(),
    locale.AsFFI(),
    &write);
  return output;
}

inline const capi::LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::AsFFI() const {
  return reinterpret_cast<const capi::LocaleDisplayNamesFormatter*>(this);
}

inline capi::LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::AsFFI() {
  return reinterpret_cast<capi::LocaleDisplayNamesFormatter*>(this);
}

inline const LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::FromFFI(const capi::LocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<const LocaleDisplayNamesFormatter*>(ptr);
}

inline LocaleDisplayNamesFormatter* LocaleDisplayNamesFormatter::FromFFI(capi::LocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<LocaleDisplayNamesFormatter*>(ptr);
}

inline void LocaleDisplayNamesFormatter::operator delete(void* ptr) {
  capi::ICU4XLocaleDisplayNamesFormatter_destroy(reinterpret_cast<capi::LocaleDisplayNamesFormatter*>(ptr));
}


#endif // LocaleDisplayNamesFormatter_HPP

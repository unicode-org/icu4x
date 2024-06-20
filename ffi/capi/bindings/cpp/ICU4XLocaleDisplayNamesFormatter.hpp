#ifndef ICU4XLocaleDisplayNamesFormatter_HPP
#define ICU4XLocaleDisplayNamesFormatter_HPP

#include "ICU4XLocaleDisplayNamesFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDisplayNamesOptionsV1.hpp"
#include "ICU4XLocale.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XLocaleDisplayNamesFormatter_create_result {union {ICU4XLocaleDisplayNamesFormatter* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XLocaleDisplayNamesFormatter_create_result ICU4XLocaleDisplayNamesFormatter_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDisplayNamesOptionsV1 options);
    
    void ICU4XLocaleDisplayNamesFormatter_of(const ICU4XLocaleDisplayNamesFormatter* self, const ICU4XLocale* locale, DiplomatWrite* write);
    
    
    void ICU4XLocaleDisplayNamesFormatter_destroy(ICU4XLocaleDisplayNamesFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XLocaleDisplayNamesFormatter>, ICU4XDataError> ICU4XLocaleDisplayNamesFormatter::create(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDisplayNamesOptionsV1 options) {
  auto result = capi::ICU4XLocaleDisplayNamesFormatter_create(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleDisplayNamesFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleDisplayNamesFormatter>>(std::unique_ptr<ICU4XLocaleDisplayNamesFormatter>(ICU4XLocaleDisplayNamesFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleDisplayNamesFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::string ICU4XLocaleDisplayNamesFormatter::of(const ICU4XLocale& locale) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocaleDisplayNamesFormatter_of(this->AsFFI(),
    locale.AsFFI(),
    &write);
  return output;
}

inline const capi::ICU4XLocaleDisplayNamesFormatter* ICU4XLocaleDisplayNamesFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocaleDisplayNamesFormatter*>(this);
}

inline capi::ICU4XLocaleDisplayNamesFormatter* ICU4XLocaleDisplayNamesFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocaleDisplayNamesFormatter*>(this);
}

inline const ICU4XLocaleDisplayNamesFormatter* ICU4XLocaleDisplayNamesFormatter::FromFFI(const capi::ICU4XLocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<const ICU4XLocaleDisplayNamesFormatter*>(ptr);
}

inline ICU4XLocaleDisplayNamesFormatter* ICU4XLocaleDisplayNamesFormatter::FromFFI(capi::ICU4XLocaleDisplayNamesFormatter* ptr) {
  return reinterpret_cast<ICU4XLocaleDisplayNamesFormatter*>(ptr);
}

inline void ICU4XLocaleDisplayNamesFormatter::operator delete(void* ptr) {
  capi::ICU4XLocaleDisplayNamesFormatter_destroy(reinterpret_cast<capi::ICU4XLocaleDisplayNamesFormatter*>(ptr));
}


#endif // ICU4XLocaleDisplayNamesFormatter_HPP

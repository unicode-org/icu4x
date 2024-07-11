#ifndef LocaleExpander_HPP
#define LocaleExpander_HPP

#include "LocaleExpander.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "Error.hpp"
#include "Locale.hpp"
#include "TransformResult.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleExpander_create_result {union {LocaleExpander* ok; Error err;}; bool is_ok;} ICU4XLocaleExpander_create_result;
    ICU4XLocaleExpander_create_result ICU4XLocaleExpander_create(const DataProvider* provider);
    
    typedef struct ICU4XLocaleExpander_create_extended_result {union {LocaleExpander* ok; Error err;}; bool is_ok;} ICU4XLocaleExpander_create_extended_result;
    ICU4XLocaleExpander_create_extended_result ICU4XLocaleExpander_create_extended(const DataProvider* provider);
    
    TransformResult ICU4XLocaleExpander_maximize(const LocaleExpander* self, Locale* locale);
    
    TransformResult ICU4XLocaleExpander_minimize(const LocaleExpander* self, Locale* locale);
    
    TransformResult ICU4XLocaleExpander_minimize_favor_script(const LocaleExpander* self, Locale* locale);
    
    
    void ICU4XLocaleExpander_destroy(LocaleExpander* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<LocaleExpander>, Error> LocaleExpander::create(const DataProvider& provider) {
  auto result = capi::ICU4XLocaleExpander_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Ok<std::unique_ptr<LocaleExpander>>(std::unique_ptr<LocaleExpander>(LocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LocaleExpander>, Error> LocaleExpander::create_extended(const DataProvider& provider) {
  auto result = capi::ICU4XLocaleExpander_create_extended(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Ok<std::unique_ptr<LocaleExpander>>(std::unique_ptr<LocaleExpander>(LocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline TransformResult LocaleExpander::maximize(Locale& locale) const {
  auto result = capi::ICU4XLocaleExpander_maximize(this->AsFFI(),
    locale.AsFFI());
  return TransformResult::FromFFI(result);
}

inline TransformResult LocaleExpander::minimize(Locale& locale) const {
  auto result = capi::ICU4XLocaleExpander_minimize(this->AsFFI(),
    locale.AsFFI());
  return TransformResult::FromFFI(result);
}

inline TransformResult LocaleExpander::minimize_favor_script(Locale& locale) const {
  auto result = capi::ICU4XLocaleExpander_minimize_favor_script(this->AsFFI(),
    locale.AsFFI());
  return TransformResult::FromFFI(result);
}

inline const capi::LocaleExpander* LocaleExpander::AsFFI() const {
  return reinterpret_cast<const capi::LocaleExpander*>(this);
}

inline capi::LocaleExpander* LocaleExpander::AsFFI() {
  return reinterpret_cast<capi::LocaleExpander*>(this);
}

inline const LocaleExpander* LocaleExpander::FromFFI(const capi::LocaleExpander* ptr) {
  return reinterpret_cast<const LocaleExpander*>(ptr);
}

inline LocaleExpander* LocaleExpander::FromFFI(capi::LocaleExpander* ptr) {
  return reinterpret_cast<LocaleExpander*>(ptr);
}

inline void LocaleExpander::operator delete(void* ptr) {
  capi::ICU4XLocaleExpander_destroy(reinterpret_cast<capi::LocaleExpander*>(ptr));
}


#endif // LocaleExpander_HPP

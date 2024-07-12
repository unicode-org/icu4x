#ifndef LocaleCanonicalizer_HPP
#define LocaleCanonicalizer_HPP

#include "LocaleCanonicalizer.d.hpp"

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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleCanonicalizer_create_result {union {diplomat::capi::LocaleCanonicalizer* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XLocaleCanonicalizer_create_result;
    ICU4XLocaleCanonicalizer_create_result ICU4XLocaleCanonicalizer_create(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XLocaleCanonicalizer_create_extended_result {union {diplomat::capi::LocaleCanonicalizer* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XLocaleCanonicalizer_create_extended_result;
    ICU4XLocaleCanonicalizer_create_extended_result ICU4XLocaleCanonicalizer_create_extended(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::TransformResult ICU4XLocaleCanonicalizer_canonicalize(const diplomat::capi::LocaleCanonicalizer* self, diplomat::capi::Locale* locale);
    
    
    void ICU4XLocaleCanonicalizer_destroy(LocaleCanonicalizer* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> LocaleCanonicalizer::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XLocaleCanonicalizer_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Ok<std::unique_ptr<LocaleCanonicalizer>>(std::unique_ptr<LocaleCanonicalizer>(LocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> LocaleCanonicalizer::create_extended(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XLocaleCanonicalizer_create_extended(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Ok<std::unique_ptr<LocaleCanonicalizer>>(std::unique_ptr<LocaleCanonicalizer>(LocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline TransformResult LocaleCanonicalizer::canonicalize(Locale& locale) const {
  auto result = diplomat::capi::ICU4XLocaleCanonicalizer_canonicalize(this->AsFFI(),
    locale.AsFFI());
  return TransformResult::FromFFI(result);
}

inline const diplomat::capi::LocaleCanonicalizer* LocaleCanonicalizer::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LocaleCanonicalizer*>(this);
}

inline diplomat::capi::LocaleCanonicalizer* LocaleCanonicalizer::AsFFI() {
  return reinterpret_cast<diplomat::capi::LocaleCanonicalizer*>(this);
}

inline const LocaleCanonicalizer* LocaleCanonicalizer::FromFFI(const diplomat::capi::LocaleCanonicalizer* ptr) {
  return reinterpret_cast<const LocaleCanonicalizer*>(ptr);
}

inline LocaleCanonicalizer* LocaleCanonicalizer::FromFFI(diplomat::capi::LocaleCanonicalizer* ptr) {
  return reinterpret_cast<LocaleCanonicalizer*>(ptr);
}

inline void LocaleCanonicalizer::operator delete(void* ptr) {
  diplomat::capi::ICU4XLocaleCanonicalizer_destroy(reinterpret_cast<diplomat::capi::LocaleCanonicalizer*>(ptr));
}


#endif // LocaleCanonicalizer_HPP

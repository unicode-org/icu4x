#ifndef LocaleCanonicalizer_HPP
#define LocaleCanonicalizer_HPP

#include "LocaleCanonicalizer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataProvider.hpp"
#include "Error.hpp"
#include "Locale.hpp"
#include "TransformResult.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_LocaleCanonicalizer_create_mv1_result {union {diplomat::capi::LocaleCanonicalizer* ok; diplomat::capi::Error err;}; bool is_ok;} icu4x_LocaleCanonicalizer_create_mv1_result;
    icu4x_LocaleCanonicalizer_create_mv1_result icu4x_LocaleCanonicalizer_create_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_LocaleCanonicalizer_create_extended_mv1_result {union {diplomat::capi::LocaleCanonicalizer* ok; diplomat::capi::Error err;}; bool is_ok;} icu4x_LocaleCanonicalizer_create_extended_mv1_result;
    icu4x_LocaleCanonicalizer_create_extended_mv1_result icu4x_LocaleCanonicalizer_create_extended_mv1(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::TransformResult icu4x_LocaleCanonicalizer_canonicalize_mv1(const diplomat::capi::LocaleCanonicalizer* self, diplomat::capi::Locale* locale);
    
    
    void icu4x_LocaleCanonicalizer_destroy_mv1(LocaleCanonicalizer* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> LocaleCanonicalizer::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_LocaleCanonicalizer_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Ok<std::unique_ptr<LocaleCanonicalizer>>(std::unique_ptr<LocaleCanonicalizer>(LocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> LocaleCanonicalizer::create_extended(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_LocaleCanonicalizer_create_extended_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Ok<std::unique_ptr<LocaleCanonicalizer>>(std::unique_ptr<LocaleCanonicalizer>(LocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline TransformResult LocaleCanonicalizer::canonicalize(Locale& locale) const {
  auto result = diplomat::capi::icu4x_LocaleCanonicalizer_canonicalize_mv1(this->AsFFI(),
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
  diplomat::capi::icu4x_LocaleCanonicalizer_destroy_mv1(reinterpret_cast<diplomat::capi::LocaleCanonicalizer*>(ptr));
}


#endif // LocaleCanonicalizer_HPP

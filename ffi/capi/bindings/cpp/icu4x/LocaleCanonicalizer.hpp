#ifndef icu4x_LocaleCanonicalizer_HPP
#define icu4x_LocaleCanonicalizer_HPP

#include "LocaleCanonicalizer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "Error.hpp"
#include "Locale.hpp"
#include "TransformResult.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_LocaleCanonicalizer_create_mv1_result {union {icu4x::capi::LocaleCanonicalizer* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_LocaleCanonicalizer_create_mv1_result;
    icu4x_LocaleCanonicalizer_create_mv1_result icu4x_LocaleCanonicalizer_create_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_LocaleCanonicalizer_create_extended_mv1_result {union {icu4x::capi::LocaleCanonicalizer* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_LocaleCanonicalizer_create_extended_mv1_result;
    icu4x_LocaleCanonicalizer_create_extended_mv1_result icu4x_LocaleCanonicalizer_create_extended_mv1(const icu4x::capi::DataProvider* provider);
    
    icu4x::capi::TransformResult icu4x_LocaleCanonicalizer_canonicalize_mv1(const icu4x::capi::LocaleCanonicalizer* self, icu4x::capi::Locale* locale);
    
    
    void icu4x_LocaleCanonicalizer_destroy_mv1(LocaleCanonicalizer* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::LocaleCanonicalizer>, icu4x::Error> icu4x::LocaleCanonicalizer::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_LocaleCanonicalizer_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::LocaleCanonicalizer>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::LocaleCanonicalizer>>(std::unique_ptr<icu4x::LocaleCanonicalizer>(icu4x::LocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::LocaleCanonicalizer>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::LocaleCanonicalizer>, icu4x::Error> icu4x::LocaleCanonicalizer::create_extended(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_LocaleCanonicalizer_create_extended_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::LocaleCanonicalizer>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::LocaleCanonicalizer>>(std::unique_ptr<icu4x::LocaleCanonicalizer>(icu4x::LocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::LocaleCanonicalizer>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline icu4x::TransformResult icu4x::LocaleCanonicalizer::canonicalize(icu4x::Locale& locale) const {
  auto result = icu4x::capi::icu4x_LocaleCanonicalizer_canonicalize_mv1(this->AsFFI(),
    locale.AsFFI());
  return icu4x::TransformResult::FromFFI(result);
}

inline const icu4x::capi::LocaleCanonicalizer* icu4x::LocaleCanonicalizer::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::LocaleCanonicalizer*>(this);
}

inline icu4x::capi::LocaleCanonicalizer* icu4x::LocaleCanonicalizer::AsFFI() {
  return reinterpret_cast<icu4x::capi::LocaleCanonicalizer*>(this);
}

inline const icu4x::LocaleCanonicalizer* icu4x::LocaleCanonicalizer::FromFFI(const icu4x::capi::LocaleCanonicalizer* ptr) {
  return reinterpret_cast<const icu4x::LocaleCanonicalizer*>(ptr);
}

inline icu4x::LocaleCanonicalizer* icu4x::LocaleCanonicalizer::FromFFI(icu4x::capi::LocaleCanonicalizer* ptr) {
  return reinterpret_cast<icu4x::LocaleCanonicalizer*>(ptr);
}

inline void icu4x::LocaleCanonicalizer::operator delete(void* ptr) {
  icu4x::capi::icu4x_LocaleCanonicalizer_destroy_mv1(reinterpret_cast<icu4x::capi::LocaleCanonicalizer*>(ptr));
}


#endif // icu4x_LocaleCanonicalizer_HPP

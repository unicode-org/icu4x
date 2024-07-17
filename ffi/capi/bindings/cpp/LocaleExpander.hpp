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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_LocaleExpander_create_mv1_result {union {diplomat::capi::LocaleExpander* ok; diplomat::capi::Error err;}; bool is_ok;} icu4x_LocaleExpander_create_mv1_result;
    icu4x_LocaleExpander_create_mv1_result icu4x_LocaleExpander_create_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_LocaleExpander_create_extended_mv1_result {union {diplomat::capi::LocaleExpander* ok; diplomat::capi::Error err;}; bool is_ok;} icu4x_LocaleExpander_create_extended_mv1_result;
    icu4x_LocaleExpander_create_extended_mv1_result icu4x_LocaleExpander_create_extended_mv1(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::TransformResult icu4x_LocaleExpander_maximize_mv1(const diplomat::capi::LocaleExpander* self, diplomat::capi::Locale* locale);
    
    diplomat::capi::TransformResult icu4x_LocaleExpander_minimize_mv1(const diplomat::capi::LocaleExpander* self, diplomat::capi::Locale* locale);
    
    diplomat::capi::TransformResult icu4x_LocaleExpander_minimize_favor_script_mv1(const diplomat::capi::LocaleExpander* self, diplomat::capi::Locale* locale);
    
    
    void icu4x_LocaleExpander_destroy_mv1(LocaleExpander* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<LocaleExpander>, Error> LocaleExpander::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_LocaleExpander_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Ok<std::unique_ptr<LocaleExpander>>(std::unique_ptr<LocaleExpander>(LocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LocaleExpander>, Error> LocaleExpander::create_extended(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_LocaleExpander_create_extended_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Ok<std::unique_ptr<LocaleExpander>>(std::unique_ptr<LocaleExpander>(LocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleExpander>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline TransformResult LocaleExpander::maximize(Locale& locale) const {
  auto result = diplomat::capi::icu4x_LocaleExpander_maximize_mv1(this->AsFFI(),
    locale.AsFFI());
  return TransformResult::FromFFI(result);
}

inline TransformResult LocaleExpander::minimize(Locale& locale) const {
  auto result = diplomat::capi::icu4x_LocaleExpander_minimize_mv1(this->AsFFI(),
    locale.AsFFI());
  return TransformResult::FromFFI(result);
}

inline TransformResult LocaleExpander::minimize_favor_script(Locale& locale) const {
  auto result = diplomat::capi::icu4x_LocaleExpander_minimize_favor_script_mv1(this->AsFFI(),
    locale.AsFFI());
  return TransformResult::FromFFI(result);
}

inline const diplomat::capi::LocaleExpander* LocaleExpander::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LocaleExpander*>(this);
}

inline diplomat::capi::LocaleExpander* LocaleExpander::AsFFI() {
  return reinterpret_cast<diplomat::capi::LocaleExpander*>(this);
}

inline const LocaleExpander* LocaleExpander::FromFFI(const diplomat::capi::LocaleExpander* ptr) {
  return reinterpret_cast<const LocaleExpander*>(ptr);
}

inline LocaleExpander* LocaleExpander::FromFFI(diplomat::capi::LocaleExpander* ptr) {
  return reinterpret_cast<LocaleExpander*>(ptr);
}

inline void LocaleExpander::operator delete(void* ptr) {
  diplomat::capi::icu4x_LocaleExpander_destroy_mv1(reinterpret_cast<diplomat::capi::LocaleExpander*>(ptr));
}


#endif // LocaleExpander_HPP

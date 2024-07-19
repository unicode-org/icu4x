#ifndef icu4x_LocaleExpander_HPP
#define icu4x_LocaleExpander_HPP

#include "LocaleExpander.d.hpp"

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
    
    typedef struct icu4x_LocaleExpander_create_mv1_result {union {icu4x::capi::LocaleExpander* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_LocaleExpander_create_mv1_result;
    icu4x_LocaleExpander_create_mv1_result icu4x_LocaleExpander_create_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_LocaleExpander_create_extended_mv1_result {union {icu4x::capi::LocaleExpander* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_LocaleExpander_create_extended_mv1_result;
    icu4x_LocaleExpander_create_extended_mv1_result icu4x_LocaleExpander_create_extended_mv1(const icu4x::capi::DataProvider* provider);
    
    icu4x::capi::TransformResult icu4x_LocaleExpander_maximize_mv1(const icu4x::capi::LocaleExpander* self, icu4x::capi::Locale* locale);
    
    icu4x::capi::TransformResult icu4x_LocaleExpander_minimize_mv1(const icu4x::capi::LocaleExpander* self, icu4x::capi::Locale* locale);
    
    icu4x::capi::TransformResult icu4x_LocaleExpander_minimize_favor_script_mv1(const icu4x::capi::LocaleExpander* self, icu4x::capi::Locale* locale);
    
    
    void icu4x_LocaleExpander_destroy_mv1(LocaleExpander* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::LocaleExpander>, icu4x::Error> icu4x::LocaleExpander::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_LocaleExpander_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::LocaleExpander>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::LocaleExpander>>(std::unique_ptr<icu4x::LocaleExpander>(icu4x::LocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::LocaleExpander>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::LocaleExpander>, icu4x::Error> icu4x::LocaleExpander::create_extended(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_LocaleExpander_create_extended_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::LocaleExpander>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::LocaleExpander>>(std::unique_ptr<icu4x::LocaleExpander>(icu4x::LocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::LocaleExpander>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline icu4x::TransformResult icu4x::LocaleExpander::maximize(icu4x::Locale& locale) const {
  auto result = icu4x::capi::icu4x_LocaleExpander_maximize_mv1(this->AsFFI(),
    locale.AsFFI());
  return icu4x::TransformResult::FromFFI(result);
}

inline icu4x::TransformResult icu4x::LocaleExpander::minimize(icu4x::Locale& locale) const {
  auto result = icu4x::capi::icu4x_LocaleExpander_minimize_mv1(this->AsFFI(),
    locale.AsFFI());
  return icu4x::TransformResult::FromFFI(result);
}

inline icu4x::TransformResult icu4x::LocaleExpander::minimize_favor_script(icu4x::Locale& locale) const {
  auto result = icu4x::capi::icu4x_LocaleExpander_minimize_favor_script_mv1(this->AsFFI(),
    locale.AsFFI());
  return icu4x::TransformResult::FromFFI(result);
}

inline const icu4x::capi::LocaleExpander* icu4x::LocaleExpander::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::LocaleExpander*>(this);
}

inline icu4x::capi::LocaleExpander* icu4x::LocaleExpander::AsFFI() {
  return reinterpret_cast<icu4x::capi::LocaleExpander*>(this);
}

inline const icu4x::LocaleExpander* icu4x::LocaleExpander::FromFFI(const icu4x::capi::LocaleExpander* ptr) {
  return reinterpret_cast<const icu4x::LocaleExpander*>(ptr);
}

inline icu4x::LocaleExpander* icu4x::LocaleExpander::FromFFI(icu4x::capi::LocaleExpander* ptr) {
  return reinterpret_cast<icu4x::LocaleExpander*>(ptr);
}

inline void icu4x::LocaleExpander::operator delete(void* ptr) {
  icu4x::capi::icu4x_LocaleExpander_destroy_mv1(reinterpret_cast<icu4x::capi::LocaleExpander*>(ptr));
}


#endif // icu4x_LocaleExpander_HPP

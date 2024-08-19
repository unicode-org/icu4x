#ifndef icu4x_LocaleDirectionality_HPP
#define icu4x_LocaleDirectionality_HPP

#include "LocaleDirectionality.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"
#include "LocaleDirection.hpp"
#include "LocaleExpander.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_LocaleDirectionality_create_mv1_result {union {icu4x::capi::LocaleDirectionality* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_LocaleDirectionality_create_mv1_result;
    icu4x_LocaleDirectionality_create_mv1_result icu4x_LocaleDirectionality_create_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_LocaleDirectionality_create_with_expander_mv1_result {union {icu4x::capi::LocaleDirectionality* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_LocaleDirectionality_create_with_expander_mv1_result;
    icu4x_LocaleDirectionality_create_with_expander_mv1_result icu4x_LocaleDirectionality_create_with_expander_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::LocaleExpander* expander);
    
    icu4x::capi::LocaleDirection icu4x_LocaleDirectionality_get_mv1(const icu4x::capi::LocaleDirectionality* self, const icu4x::capi::Locale* locale);
    
    bool icu4x_LocaleDirectionality_is_left_to_right_mv1(const icu4x::capi::LocaleDirectionality* self, const icu4x::capi::Locale* locale);
    
    bool icu4x_LocaleDirectionality_is_right_to_left_mv1(const icu4x::capi::LocaleDirectionality* self, const icu4x::capi::Locale* locale);
    
    
    void icu4x_LocaleDirectionality_destroy_mv1(LocaleDirectionality* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::LocaleDirectionality>, icu4x::DataError> icu4x::LocaleDirectionality::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_LocaleDirectionality_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::LocaleDirectionality>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::LocaleDirectionality>>(std::unique_ptr<icu4x::LocaleDirectionality>(icu4x::LocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::LocaleDirectionality>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::LocaleDirectionality>, icu4x::DataError> icu4x::LocaleDirectionality::create_with_expander(const icu4x::DataProvider& provider, const icu4x::LocaleExpander& expander) {
  auto result = icu4x::capi::icu4x_LocaleDirectionality_create_with_expander_mv1(provider.AsFFI(),
    expander.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::LocaleDirectionality>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::LocaleDirectionality>>(std::unique_ptr<icu4x::LocaleDirectionality>(icu4x::LocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::LocaleDirectionality>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::LocaleDirection icu4x::LocaleDirectionality::get(const icu4x::Locale& locale) const {
  auto result = icu4x::capi::icu4x_LocaleDirectionality_get_mv1(this->AsFFI(),
    locale.AsFFI());
  return icu4x::LocaleDirection::FromFFI(result);
}

inline bool icu4x::LocaleDirectionality::is_left_to_right(const icu4x::Locale& locale) const {
  auto result = icu4x::capi::icu4x_LocaleDirectionality_is_left_to_right_mv1(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline bool icu4x::LocaleDirectionality::is_right_to_left(const icu4x::Locale& locale) const {
  auto result = icu4x::capi::icu4x_LocaleDirectionality_is_right_to_left_mv1(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline const icu4x::capi::LocaleDirectionality* icu4x::LocaleDirectionality::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::LocaleDirectionality*>(this);
}

inline icu4x::capi::LocaleDirectionality* icu4x::LocaleDirectionality::AsFFI() {
  return reinterpret_cast<icu4x::capi::LocaleDirectionality*>(this);
}

inline const icu4x::LocaleDirectionality* icu4x::LocaleDirectionality::FromFFI(const icu4x::capi::LocaleDirectionality* ptr) {
  return reinterpret_cast<const icu4x::LocaleDirectionality*>(ptr);
}

inline icu4x::LocaleDirectionality* icu4x::LocaleDirectionality::FromFFI(icu4x::capi::LocaleDirectionality* ptr) {
  return reinterpret_cast<icu4x::LocaleDirectionality*>(ptr);
}

inline void icu4x::LocaleDirectionality::operator delete(void* ptr) {
  icu4x::capi::icu4x_LocaleDirectionality_destroy_mv1(reinterpret_cast<icu4x::capi::LocaleDirectionality*>(ptr));
}


#endif // icu4x_LocaleDirectionality_HPP

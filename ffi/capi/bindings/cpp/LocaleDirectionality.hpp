#ifndef LocaleDirectionality_HPP
#define LocaleDirectionality_HPP

#include "LocaleDirectionality.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"
#include "LocaleDirection.hpp"
#include "LocaleExpander.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleDirectionality_create_result {union {diplomat::capi::LocaleDirectionality* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XLocaleDirectionality_create_result;
    ICU4XLocaleDirectionality_create_result ICU4XLocaleDirectionality_create(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XLocaleDirectionality_create_with_expander_result {union {diplomat::capi::LocaleDirectionality* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XLocaleDirectionality_create_with_expander_result;
    ICU4XLocaleDirectionality_create_with_expander_result ICU4XLocaleDirectionality_create_with_expander(const diplomat::capi::DataProvider* provider, const diplomat::capi::LocaleExpander* expander);
    
    diplomat::capi::LocaleDirection ICU4XLocaleDirectionality_get(const diplomat::capi::LocaleDirectionality* self, const diplomat::capi::Locale* locale);
    
    bool ICU4XLocaleDirectionality_is_left_to_right(const diplomat::capi::LocaleDirectionality* self, const diplomat::capi::Locale* locale);
    
    bool ICU4XLocaleDirectionality_is_right_to_left(const diplomat::capi::LocaleDirectionality* self, const diplomat::capi::Locale* locale);
    
    
    void ICU4XLocaleDirectionality_destroy(LocaleDirectionality* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError> LocaleDirectionality::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XLocaleDirectionality_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Ok<std::unique_ptr<LocaleDirectionality>>(std::unique_ptr<LocaleDirectionality>(LocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError> LocaleDirectionality::create_with_expander(const DataProvider& provider, const LocaleExpander& expander) {
  auto result = diplomat::capi::ICU4XLocaleDirectionality_create_with_expander(provider.AsFFI(),
    expander.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Ok<std::unique_ptr<LocaleDirectionality>>(std::unique_ptr<LocaleDirectionality>(LocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline LocaleDirection LocaleDirectionality::get(const Locale& locale) const {
  auto result = diplomat::capi::ICU4XLocaleDirectionality_get(this->AsFFI(),
    locale.AsFFI());
  return LocaleDirection::FromFFI(result);
}

inline bool LocaleDirectionality::is_left_to_right(const Locale& locale) const {
  auto result = diplomat::capi::ICU4XLocaleDirectionality_is_left_to_right(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline bool LocaleDirectionality::is_right_to_left(const Locale& locale) const {
  auto result = diplomat::capi::ICU4XLocaleDirectionality_is_right_to_left(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline const diplomat::capi::LocaleDirectionality* LocaleDirectionality::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LocaleDirectionality*>(this);
}

inline diplomat::capi::LocaleDirectionality* LocaleDirectionality::AsFFI() {
  return reinterpret_cast<diplomat::capi::LocaleDirectionality*>(this);
}

inline const LocaleDirectionality* LocaleDirectionality::FromFFI(const diplomat::capi::LocaleDirectionality* ptr) {
  return reinterpret_cast<const LocaleDirectionality*>(ptr);
}

inline LocaleDirectionality* LocaleDirectionality::FromFFI(diplomat::capi::LocaleDirectionality* ptr) {
  return reinterpret_cast<LocaleDirectionality*>(ptr);
}

inline void LocaleDirectionality::operator delete(void* ptr) {
  diplomat::capi::ICU4XLocaleDirectionality_destroy(reinterpret_cast<diplomat::capi::LocaleDirectionality*>(ptr));
}


#endif // LocaleDirectionality_HPP

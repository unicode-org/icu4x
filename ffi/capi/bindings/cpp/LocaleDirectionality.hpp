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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleDirectionality_create_result {union {LocaleDirectionality* ok; DataError err;}; bool is_ok;} ICU4XLocaleDirectionality_create_result;
    ICU4XLocaleDirectionality_create_result ICU4XLocaleDirectionality_create(const DataProvider* provider);
    
    typedef struct ICU4XLocaleDirectionality_create_with_expander_result {union {LocaleDirectionality* ok; DataError err;}; bool is_ok;} ICU4XLocaleDirectionality_create_with_expander_result;
    ICU4XLocaleDirectionality_create_with_expander_result ICU4XLocaleDirectionality_create_with_expander(const DataProvider* provider, const LocaleExpander* expander);
    
    LocaleDirection ICU4XLocaleDirectionality_get(const LocaleDirectionality* self, const Locale* locale);
    
    bool ICU4XLocaleDirectionality_is_left_to_right(const LocaleDirectionality* self, const Locale* locale);
    
    bool ICU4XLocaleDirectionality_is_right_to_left(const LocaleDirectionality* self, const Locale* locale);
    
    
    void ICU4XLocaleDirectionality_destroy(LocaleDirectionality* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError> LocaleDirectionality::create(const DataProvider& provider) {
  auto result = capi::ICU4XLocaleDirectionality_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Ok<std::unique_ptr<LocaleDirectionality>>(std::unique_ptr<LocaleDirectionality>(LocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError> LocaleDirectionality::create_with_expander(const DataProvider& provider, const LocaleExpander& expander) {
  auto result = capi::ICU4XLocaleDirectionality_create_with_expander(provider.AsFFI(),
    expander.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Ok<std::unique_ptr<LocaleDirectionality>>(std::unique_ptr<LocaleDirectionality>(LocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline LocaleDirection LocaleDirectionality::get(const Locale& locale) const {
  auto result = capi::ICU4XLocaleDirectionality_get(this->AsFFI(),
    locale.AsFFI());
  return LocaleDirection::FromFFI(result);
}

inline bool LocaleDirectionality::is_left_to_right(const Locale& locale) const {
  auto result = capi::ICU4XLocaleDirectionality_is_left_to_right(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline bool LocaleDirectionality::is_right_to_left(const Locale& locale) const {
  auto result = capi::ICU4XLocaleDirectionality_is_right_to_left(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline const capi::LocaleDirectionality* LocaleDirectionality::AsFFI() const {
  return reinterpret_cast<const capi::LocaleDirectionality*>(this);
}

inline capi::LocaleDirectionality* LocaleDirectionality::AsFFI() {
  return reinterpret_cast<capi::LocaleDirectionality*>(this);
}

inline const LocaleDirectionality* LocaleDirectionality::FromFFI(const capi::LocaleDirectionality* ptr) {
  return reinterpret_cast<const LocaleDirectionality*>(ptr);
}

inline LocaleDirectionality* LocaleDirectionality::FromFFI(capi::LocaleDirectionality* ptr) {
  return reinterpret_cast<LocaleDirectionality*>(ptr);
}

inline void LocaleDirectionality::operator delete(void* ptr) {
  capi::ICU4XLocaleDirectionality_destroy(reinterpret_cast<capi::LocaleDirectionality*>(ptr));
}


#endif // LocaleDirectionality_HPP

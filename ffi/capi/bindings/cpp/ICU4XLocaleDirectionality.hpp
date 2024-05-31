#ifndef ICU4XLocaleDirectionality_HPP
#define ICU4XLocaleDirectionality_HPP

#include "ICU4XLocaleDirectionality.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XLocaleDirection.hpp"
#include "ICU4XLocaleDirectionality.h"
#include "ICU4XLocaleExpander.hpp"


inline diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError> ICU4XLocaleDirectionality::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLocaleDirectionality_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleDirectionality>>(std::unique_ptr<ICU4XLocaleDirectionality>(ICU4XLocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError> ICU4XLocaleDirectionality::create_with_expander(const ICU4XDataProvider& provider, const ICU4XLocaleExpander& expander) {
  auto result = capi::ICU4XLocaleDirectionality_create_with_expander(provider.AsFFI(),
    expander.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleDirectionality>>(std::unique_ptr<ICU4XLocaleDirectionality>(ICU4XLocaleDirectionality::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline ICU4XLocaleDirection ICU4XLocaleDirectionality::get(const ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleDirectionality_get(this->AsFFI(),
    locale.AsFFI());
  return ICU4XLocaleDirection::FromFFI(result);
}

inline bool ICU4XLocaleDirectionality::is_left_to_right(const ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleDirectionality_is_left_to_right(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline bool ICU4XLocaleDirectionality::is_right_to_left(const ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleDirectionality_is_right_to_left(this->AsFFI(),
    locale.AsFFI());
  return result;
}

inline const capi::ICU4XLocaleDirectionality* ICU4XLocaleDirectionality::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocaleDirectionality*>(this);
}

inline capi::ICU4XLocaleDirectionality* ICU4XLocaleDirectionality::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocaleDirectionality*>(this);
}

inline const ICU4XLocaleDirectionality* ICU4XLocaleDirectionality::FromFFI(const capi::ICU4XLocaleDirectionality* ptr) {
  return reinterpret_cast<const ICU4XLocaleDirectionality*>(ptr);
}

inline ICU4XLocaleDirectionality* ICU4XLocaleDirectionality::FromFFI(capi::ICU4XLocaleDirectionality* ptr) {
  return reinterpret_cast<ICU4XLocaleDirectionality*>(ptr);
}

inline void ICU4XLocaleDirectionality::operator delete(void* ptr) {
  capi::ICU4XLocaleDirectionality_destroy(reinterpret_cast<capi::ICU4XLocaleDirectionality*>(ptr));
}


#endif // ICU4XLocaleDirectionality_HPP

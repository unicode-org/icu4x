#ifndef ICU4XLocaleExpander_HPP
#define ICU4XLocaleExpander_HPP

#include "ICU4XLocaleExpander.d.hpp"

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
#include "ICU4XLocaleExpander.h"
#include "ICU4XTransformResult.hpp"


inline diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError> ICU4XLocaleExpander::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLocaleExpander_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleExpander>>(std::unique_ptr<ICU4XLocaleExpander>(ICU4XLocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError> ICU4XLocaleExpander::create_extended(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLocaleExpander_create_extended(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleExpander>>(std::unique_ptr<ICU4XLocaleExpander>(ICU4XLocaleExpander::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline ICU4XTransformResult ICU4XLocaleExpander::maximize(ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleExpander_maximize(this->AsFFI(),
    locale.AsFFI());
  return ICU4XTransformResult::FromFFI(result);
}

inline ICU4XTransformResult ICU4XLocaleExpander::minimize(ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleExpander_minimize(this->AsFFI(),
    locale.AsFFI());
  return ICU4XTransformResult::FromFFI(result);
}

inline ICU4XTransformResult ICU4XLocaleExpander::minimize_favor_script(ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleExpander_minimize_favor_script(this->AsFFI(),
    locale.AsFFI());
  return ICU4XTransformResult::FromFFI(result);
}

inline const capi::ICU4XLocaleExpander* ICU4XLocaleExpander::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocaleExpander*>(this);
}

inline capi::ICU4XLocaleExpander* ICU4XLocaleExpander::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocaleExpander*>(this);
}

inline const ICU4XLocaleExpander* ICU4XLocaleExpander::FromFFI(const capi::ICU4XLocaleExpander* ptr) {
  return reinterpret_cast<const ICU4XLocaleExpander*>(ptr);
}

inline ICU4XLocaleExpander* ICU4XLocaleExpander::FromFFI(capi::ICU4XLocaleExpander* ptr) {
  return reinterpret_cast<ICU4XLocaleExpander*>(ptr);
}

inline void ICU4XLocaleExpander::operator delete(void* ptr) {
  capi::ICU4XLocaleExpander_destroy(reinterpret_cast<capi::ICU4XLocaleExpander*>(ptr));
}


#endif // ICU4XLocaleExpander_HPP

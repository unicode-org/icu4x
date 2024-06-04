#ifndef ICU4XPluralRules_HPP
#define ICU4XPluralRules_HPP

#include "ICU4XPluralRules.d.hpp"

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
#include "ICU4XPluralCategories.hpp"
#include "ICU4XPluralCategory.hpp"
#include "ICU4XPluralOperands.hpp"
#include "ICU4XPluralRules.h"


inline diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError> ICU4XPluralRules::create_cardinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XPluralRules_create_cardinal(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XPluralRules>>(std::unique_ptr<ICU4XPluralRules>(ICU4XPluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError> ICU4XPluralRules::create_ordinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XPluralRules_create_ordinal(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XPluralRules>>(std::unique_ptr<ICU4XPluralRules>(ICU4XPluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline ICU4XPluralCategory ICU4XPluralRules::category_for(const ICU4XPluralOperands& op) const {
  auto result = capi::ICU4XPluralRules_category_for(this->AsFFI(),
    op.AsFFI());
  return ICU4XPluralCategory::FromFFI(result);
}

inline ICU4XPluralCategories ICU4XPluralRules::categories() const {
  auto result = capi::ICU4XPluralRules_categories(this->AsFFI());
  return ICU4XPluralCategories::FromFFI(result);
}

inline const capi::ICU4XPluralRules* ICU4XPluralRules::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XPluralRules*>(this);
}

inline capi::ICU4XPluralRules* ICU4XPluralRules::AsFFI() {
  return reinterpret_cast<capi::ICU4XPluralRules*>(this);
}

inline const ICU4XPluralRules* ICU4XPluralRules::FromFFI(const capi::ICU4XPluralRules* ptr) {
  return reinterpret_cast<const ICU4XPluralRules*>(ptr);
}

inline ICU4XPluralRules* ICU4XPluralRules::FromFFI(capi::ICU4XPluralRules* ptr) {
  return reinterpret_cast<ICU4XPluralRules*>(ptr);
}

inline void ICU4XPluralRules::operator delete(void* ptr) {
  capi::ICU4XPluralRules_destroy(reinterpret_cast<capi::ICU4XPluralRules*>(ptr));
}


#endif // ICU4XPluralRules_HPP

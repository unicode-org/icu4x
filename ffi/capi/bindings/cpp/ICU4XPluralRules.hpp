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
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XPluralCategories.hpp"
#include "ICU4XPluralCategory.hpp"
#include "ICU4XPluralOperands.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XPluralRules_create_cardinal_result {union {ICU4XPluralRules* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPluralRules_create_cardinal_result;
    ICU4XPluralRules_create_cardinal_result ICU4XPluralRules_create_cardinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);
    
    typedef struct ICU4XPluralRules_create_ordinal_result {union {ICU4XPluralRules* ok; ICU4XDataError err;}; bool is_ok;} ICU4XPluralRules_create_ordinal_result;
    ICU4XPluralRules_create_ordinal_result ICU4XPluralRules_create_ordinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);
    
    ICU4XPluralCategory ICU4XPluralRules_category_for(const ICU4XPluralRules* self, const ICU4XPluralOperands* op);
    
    ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);
    
    
    void ICU4XPluralRules_destroy(ICU4XPluralRules* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XDataError> ICU4XPluralRules::create_cardinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XPluralRules_create_cardinal(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPluralRules>>(std::unique_ptr<ICU4XPluralRules>(ICU4XPluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XDataError> ICU4XPluralRules::create_ordinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XPluralRules_create_ordinal(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XPluralRules>>(std::unique_ptr<ICU4XPluralRules>(ICU4XPluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
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

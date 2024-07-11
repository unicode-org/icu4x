#ifndef PluralRules_HPP
#define PluralRules_HPP

#include "PluralRules.d.hpp"

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
#include "PluralCategories.hpp"
#include "PluralCategory.hpp"
#include "PluralOperands.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XPluralRules_create_cardinal_result {union {PluralRules* ok; DataError err;}; bool is_ok;} ICU4XPluralRules_create_cardinal_result;
    ICU4XPluralRules_create_cardinal_result ICU4XPluralRules_create_cardinal(const DataProvider* provider, const Locale* locale);
    
    typedef struct ICU4XPluralRules_create_ordinal_result {union {PluralRules* ok; DataError err;}; bool is_ok;} ICU4XPluralRules_create_ordinal_result;
    ICU4XPluralRules_create_ordinal_result ICU4XPluralRules_create_ordinal(const DataProvider* provider, const Locale* locale);
    
    PluralCategory ICU4XPluralRules_category_for(const PluralRules* self, const PluralOperands* op);
    
    PluralCategories ICU4XPluralRules_categories(const PluralRules* self);
    
    
    void ICU4XPluralRules_destroy(PluralRules* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<PluralRules>, DataError> PluralRules::create_cardinal(const DataProvider& provider, const Locale& locale) {
  auto result = capi::ICU4XPluralRules_create_cardinal(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Ok<std::unique_ptr<PluralRules>>(std::unique_ptr<PluralRules>(PluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PluralRules>, DataError> PluralRules::create_ordinal(const DataProvider& provider, const Locale& locale) {
  auto result = capi::ICU4XPluralRules_create_ordinal(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Ok<std::unique_ptr<PluralRules>>(std::unique_ptr<PluralRules>(PluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline PluralCategory PluralRules::category_for(const PluralOperands& op) const {
  auto result = capi::ICU4XPluralRules_category_for(this->AsFFI(),
    op.AsFFI());
  return PluralCategory::FromFFI(result);
}

inline PluralCategories PluralRules::categories() const {
  auto result = capi::ICU4XPluralRules_categories(this->AsFFI());
  return PluralCategories::FromFFI(result);
}

inline const capi::PluralRules* PluralRules::AsFFI() const {
  return reinterpret_cast<const capi::PluralRules*>(this);
}

inline capi::PluralRules* PluralRules::AsFFI() {
  return reinterpret_cast<capi::PluralRules*>(this);
}

inline const PluralRules* PluralRules::FromFFI(const capi::PluralRules* ptr) {
  return reinterpret_cast<const PluralRules*>(ptr);
}

inline PluralRules* PluralRules::FromFFI(capi::PluralRules* ptr) {
  return reinterpret_cast<PluralRules*>(ptr);
}

inline void PluralRules::operator delete(void* ptr) {
  capi::ICU4XPluralRules_destroy(reinterpret_cast<capi::PluralRules*>(ptr));
}


#endif // PluralRules_HPP

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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_PluralRules_create_cardinal_mv1_result {union {diplomat::capi::PluralRules* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PluralRules_create_cardinal_mv1_result;
    icu4x_PluralRules_create_cardinal_mv1_result icu4x_PluralRules_create_cardinal_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    typedef struct icu4x_PluralRules_create_ordinal_mv1_result {union {diplomat::capi::PluralRules* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_PluralRules_create_ordinal_mv1_result;
    icu4x_PluralRules_create_ordinal_mv1_result icu4x_PluralRules_create_ordinal_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    diplomat::capi::PluralCategory icu4x_PluralRules_category_for_mv1(const diplomat::capi::PluralRules* self, const diplomat::capi::PluralOperands* op);
    
    diplomat::capi::PluralCategories icu4x_PluralRules_categories_mv1(const diplomat::capi::PluralRules* self);
    
    
    void icu4x_PluralRules_destroy_mv1(PluralRules* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<PluralRules>, DataError> PluralRules::create_cardinal(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::icu4x_PluralRules_create_cardinal_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Ok<std::unique_ptr<PluralRules>>(std::unique_ptr<PluralRules>(PluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<PluralRules>, DataError> PluralRules::create_ordinal(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::icu4x_PluralRules_create_ordinal_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Ok<std::unique_ptr<PluralRules>>(std::unique_ptr<PluralRules>(PluralRules::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PluralRules>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline PluralCategory PluralRules::category_for(const PluralOperands& op) const {
  auto result = diplomat::capi::icu4x_PluralRules_category_for_mv1(this->AsFFI(),
    op.AsFFI());
  return PluralCategory::FromFFI(result);
}

inline PluralCategories PluralRules::categories() const {
  auto result = diplomat::capi::icu4x_PluralRules_categories_mv1(this->AsFFI());
  return PluralCategories::FromFFI(result);
}

inline const diplomat::capi::PluralRules* PluralRules::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::PluralRules*>(this);
}

inline diplomat::capi::PluralRules* PluralRules::AsFFI() {
  return reinterpret_cast<diplomat::capi::PluralRules*>(this);
}

inline const PluralRules* PluralRules::FromFFI(const diplomat::capi::PluralRules* ptr) {
  return reinterpret_cast<const PluralRules*>(ptr);
}

inline PluralRules* PluralRules::FromFFI(diplomat::capi::PluralRules* ptr) {
  return reinterpret_cast<PluralRules*>(ptr);
}

inline void PluralRules::operator delete(void* ptr) {
  diplomat::capi::icu4x_PluralRules_destroy_mv1(reinterpret_cast<diplomat::capi::PluralRules*>(ptr));
}


#endif // PluralRules_HPP

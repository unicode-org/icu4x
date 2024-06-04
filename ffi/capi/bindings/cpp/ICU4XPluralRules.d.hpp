#ifndef ICU4XPluralRules_D_HPP
#define ICU4XPluralRules_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XPluralCategories.d.hpp"
#include "ICU4XPluralCategory.d.hpp"
#include "ICU4XPluralRules.d.h"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XPluralOperands;
struct ICU4XPluralCategories;
class ICU4XError;
class ICU4XPluralCategory;


class ICU4XPluralRules {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError> create_cardinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static diplomat::result<std::unique_ptr<ICU4XPluralRules>, ICU4XError> create_ordinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline ICU4XPluralCategory category_for(const ICU4XPluralOperands& op) const;

  inline ICU4XPluralCategories categories() const;

  inline const capi::ICU4XPluralRules* AsFFI() const;
  inline capi::ICU4XPluralRules* AsFFI();
  inline static const ICU4XPluralRules* FromFFI(const capi::ICU4XPluralRules* ptr);
  inline static ICU4XPluralRules* FromFFI(capi::ICU4XPluralRules* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XPluralRules() = delete;
  ICU4XPluralRules(const ICU4XPluralRules&) = delete;
  ICU4XPluralRules(ICU4XPluralRules&&) noexcept = delete;
  ICU4XPluralRules operator=(const ICU4XPluralRules&) = delete;
  ICU4XPluralRules operator=(ICU4XPluralRules&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XPluralRules_D_HPP

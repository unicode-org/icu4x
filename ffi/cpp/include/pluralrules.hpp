// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_PLURALRULES_HPP
#define ICU4X_PLURALRULES_HPP

#include <algorithm>
#include <memory>

#include "../../capi/include/pluralrules.h"
#include "locale.hpp"
#include "provider.hpp"

namespace icu4x {
struct ICU4XPluralRulesDeleter {
  void operator()(ICU4XPluralRules* l) const noexcept {
    icu4x_plural_rules_destroy(l);
  }
};
enum class PluralRuleType {
  Cardinal = ICU4XPluralRuleType_Cardinal,
  Ordinal = ICU4XPluralRuleType_Ordinal
};
enum class PluralCategory {
  Zero = ICU4XPluralCategory_Zero,
  One = ICU4XPluralCategory_One,
  Two = ICU4XPluralCategory_Two,
  Few = ICU4XPluralCategory_Few,
  Many = ICU4XPluralCategory_Many,
  Other = ICU4XPluralCategory_Other,
};
using PluralOperands = ICU4XPluralOperands;
class PluralRules {
 public:
  static std::optional<PluralRules> Create(const Locale& locale,
                                           const DataProvider& provider,
                                           PluralRuleType ty) {
    ICU4XDataProvider dp = provider.AsFFI();
    ICU4XPluralRuleType ty_ffi = static_cast<ICU4XPluralRuleType>(ty);
    ICU4XCreatePluralRulesResult result =
        icu4x_plural_rules_create(locale.AsFFI(), &dp, ty_ffi);
    if (!result.success) {
      return {};
    }
    return PluralRules(result.rules);
  }
  PluralCategory Select(PluralOperands& op) const {
    ICU4XPluralCategory cat = icu4x_plural_rules_select(this->inner.get(), &op);
    return static_cast<PluralCategory>(cat);
  }

 private:
  PluralRules(ICU4XPluralRules* i) : inner(i) {}
  std::unique_ptr<ICU4XPluralRules, ICU4XPluralRulesDeleter> inner;
};
}  // namespace icu4x

#endif  // ICU4X_PLURALRULES_HPP

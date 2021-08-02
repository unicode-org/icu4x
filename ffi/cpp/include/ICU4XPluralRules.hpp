#ifndef ICU4XPluralRules_HPP
#define ICU4XPluralRules_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralRules.h"
}

class ICU4XLocale;
class ICU4XDataProvider;
enum struct ICU4XPluralRuleType;
struct ICU4XCreatePluralRulesResult;
struct ICU4XPluralOperands;
enum struct ICU4XPluralCategory;
struct ICU4XPluralCategories;

struct ICU4XPluralRulesDeleter {
  void operator()(capi::ICU4XPluralRules* l) const noexcept {
    capi::ICU4XPluralRules_destroy(l);
  }
};
class ICU4XPluralRules {
 public:
  static ICU4XCreatePluralRulesResult create(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XPluralRuleType ty);
  ICU4XPluralCategory select(const ICU4XPluralOperands& op);
  ICU4XPluralCategories categories();
  inline const capi::ICU4XPluralRules* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XPluralRules* AsFFIMut() { return this->inner.get(); }
  ICU4XPluralRules(capi::ICU4XPluralRules* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XPluralRules, ICU4XPluralRulesDeleter> inner;
};

#include "ICU4XLocale.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XPluralRuleType.hpp"
#include "ICU4XCreatePluralRulesResult.hpp"
#include "ICU4XPluralOperands.hpp"
#include "ICU4XPluralCategory.hpp"
#include "ICU4XPluralCategories.hpp"

ICU4XCreatePluralRulesResult ICU4XPluralRules::create(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XPluralRuleType ty) {
  capi::ICU4XCreatePluralRulesResult diplomat_raw_struct_out_value = capi::ICU4XPluralRules_create(locale.AsFFI(), provider.AsFFI(), static_cast<capi::ICU4XPluralRuleType>(ty));
  auto diplomat_optional_raw_out_value_rules = diplomat_raw_struct_out_value.rules;
  std::optional<ICU4XPluralRules> diplomat_optional_out_value_rules;
  if (diplomat_optional_raw_out_value_rules != nullptr) {
    diplomat_optional_out_value_rules = ICU4XPluralRules(diplomat_optional_raw_out_value_rules);
  } else {
    diplomat_optional_out_value_rules = std::nullopt;
  }
  return ICU4XCreatePluralRulesResult{ .rules = std::move(diplomat_optional_out_value_rules), .success = std::move(diplomat_raw_struct_out_value.success) };
}
ICU4XPluralCategory ICU4XPluralRules::select(const ICU4XPluralOperands& op) {
  return ICU4XPluralCategory{ capi::ICU4XPluralRules_select(this->inner.get(), (capi::ICU4XPluralOperands*) &op) };
}
ICU4XPluralCategories ICU4XPluralRules::categories() {
  capi::ICU4XPluralCategories diplomat_raw_struct_out_value = capi::ICU4XPluralRules_categories(this->inner.get());
  return ICU4XPluralCategories{ .zero = std::move(diplomat_raw_struct_out_value.zero), .one = std::move(diplomat_raw_struct_out_value.one), .two = std::move(diplomat_raw_struct_out_value.two), .few = std::move(diplomat_raw_struct_out_value.few), .many = std::move(diplomat_raw_struct_out_value.many), .other = std::move(diplomat_raw_struct_out_value.other) };
}
#endif

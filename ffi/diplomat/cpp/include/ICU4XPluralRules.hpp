#ifndef ICU4XPluralRules_HPP
#define ICU4XPluralRules_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralRules.h"
}

class ICU4XLocale;
class ICU4XDataProvider;
#include "ICU4XPluralRuleType.hpp"
struct ICU4XCreatePluralRulesResult;
struct ICU4XPluralOperands;
#include "ICU4XPluralCategory.hpp"
struct ICU4XPluralCategories;

/**
 * A destruction policy for using ICU4XPluralRules with std::unique_ptr.
 */
struct ICU4XPluralRulesDeleter {
  void operator()(capi::ICU4XPluralRules* l) const noexcept {
    capi::ICU4XPluralRules_destroy(l);
  }
};
class ICU4XPluralRules {
 public:

  /**
   * FFI version of `PluralRules::try_new()`.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new) for more details.
   */
  static ICU4XCreatePluralRulesResult try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XPluralRuleType ty);

  /**
   * FFI version of `PluralRules::select()`.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select) for more details.
   */
  ICU4XPluralCategory select(ICU4XPluralOperands op) const;

  /**
   * FFI version of `PluralRules::categories()`.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories) for more details.
   */
  ICU4XPluralCategories categories() const;
  inline const capi::ICU4XPluralRules* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XPluralRules* AsFFIMut() { return this->inner.get(); }
  inline ICU4XPluralRules(capi::ICU4XPluralRules* i) : inner(i) {}
  ICU4XPluralRules() = default;
  ICU4XPluralRules(ICU4XPluralRules&&) noexcept = default;
  ICU4XPluralRules& operator=(ICU4XPluralRules&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XPluralRules, ICU4XPluralRulesDeleter> inner;
};

#include "ICU4XLocale.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XCreatePluralRulesResult.hpp"
#include "ICU4XPluralOperands.hpp"
#include "ICU4XPluralCategories.hpp"

inline ICU4XCreatePluralRulesResult ICU4XPluralRules::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XPluralRuleType ty) {
  capi::ICU4XCreatePluralRulesResult diplomat_raw_struct_out_value = capi::ICU4XPluralRules_try_new(locale.AsFFI(), provider.AsFFI(), static_cast<capi::ICU4XPluralRuleType>(ty));
  auto diplomat_optional_raw_out_value_rules = diplomat_raw_struct_out_value.rules;
  std::optional<ICU4XPluralRules> diplomat_optional_out_value_rules;
  if (diplomat_optional_raw_out_value_rules != nullptr) {
    diplomat_optional_out_value_rules = ICU4XPluralRules(diplomat_optional_raw_out_value_rules);
  } else {
    diplomat_optional_out_value_rules = std::nullopt;
  }
  return ICU4XCreatePluralRulesResult{ .rules = std::move(diplomat_optional_out_value_rules), .success = std::move(diplomat_raw_struct_out_value.success) };
}
inline ICU4XPluralCategory ICU4XPluralRules::select(ICU4XPluralOperands op) const {
  ICU4XPluralOperands diplomat_wrapped_struct_op = op;
  return static_cast<ICU4XPluralCategory>(capi::ICU4XPluralRules_select(this->inner.get(), capi::ICU4XPluralOperands{ .i = diplomat_wrapped_struct_op.i, .v = diplomat_wrapped_struct_op.v, .w = diplomat_wrapped_struct_op.w, .f = diplomat_wrapped_struct_op.f, .t = diplomat_wrapped_struct_op.t, .c = diplomat_wrapped_struct_op.c }));
}
inline ICU4XPluralCategories ICU4XPluralRules::categories() const {
  capi::ICU4XPluralCategories diplomat_raw_struct_out_value = capi::ICU4XPluralRules_categories(this->inner.get());
  return ICU4XPluralCategories{ .zero = std::move(diplomat_raw_struct_out_value.zero), .one = std::move(diplomat_raw_struct_out_value.one), .two = std::move(diplomat_raw_struct_out_value.two), .few = std::move(diplomat_raw_struct_out_value.few), .many = std::move(diplomat_raw_struct_out_value.many), .other = std::move(diplomat_raw_struct_out_value.other) };
}
#endif

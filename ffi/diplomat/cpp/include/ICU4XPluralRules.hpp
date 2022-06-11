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
class ICU4XPluralRules;
#include "ICU4XError.hpp"
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

/**
 * FFI version of `PluralRules`.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html) for more information.
 */
class ICU4XPluralRules {
 public:

  /**
   * FFI version of `PluralRules::try_new_cardinal()`.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XPluralRules, ICU4XError> try_new_cardinal(const ICU4XLocale& locale, const ICU4XDataProvider& provider);

  /**
   * FFI version of `PluralRules::try_new_ordinal()`.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XPluralRules, ICU4XError> try_new_ordinal(const ICU4XLocale& locale, const ICU4XDataProvider& provider);

  /**
   * FFI version of `PluralRules::select()`.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select) for more information.
   */
  ICU4XPluralCategory select(ICU4XPluralOperands op) const;

  /**
   * FFI version of `PluralRules::categories()`.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories) for more information.
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
#include "ICU4XPluralOperands.hpp"
#include "ICU4XPluralCategories.hpp"

inline diplomat::result<ICU4XPluralRules, ICU4XError> ICU4XPluralRules::try_new_cardinal(const ICU4XLocale& locale, const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XPluralRules_try_new_cardinal(locale.AsFFI(), provider.AsFFI());
  diplomat::result<ICU4XPluralRules, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XPluralRules(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XPluralRules, ICU4XError> ICU4XPluralRules::try_new_ordinal(const ICU4XLocale& locale, const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XPluralRules_try_new_ordinal(locale.AsFFI(), provider.AsFFI());
  diplomat::result<ICU4XPluralRules, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XPluralRules(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
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

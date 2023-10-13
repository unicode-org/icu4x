#ifndef ICU4XPluralRulesWithRanges_HPP
#define ICU4XPluralRulesWithRanges_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XPluralRulesWithRanges.h"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XPluralRulesWithRanges;
#include "ICU4XError.hpp"
class ICU4XPluralOperands;
#include "ICU4XPluralCategory.hpp"
struct ICU4XPluralCategories;

/**
 * A destruction policy for using ICU4XPluralRulesWithRanges with std::unique_ptr.
 */
struct ICU4XPluralRulesWithRangesDeleter {
  void operator()(capi::ICU4XPluralRulesWithRanges* l) const noexcept {
    capi::ICU4XPluralRulesWithRanges_destroy(l);
  }
};

/**
 * FFI version of `PluralRulesWithRanges`.
 * 
 * See the [Rust documentation for `PluralRulesWithRanges`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html) for more information.
 */
class ICU4XPluralRulesWithRanges {
 public:

  /**
   * Construct an [`ICU4XPluralRulesWithRanges`] for the given locale, for cardinal numbers
   * 
   * See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_cardinal) for more information.
   */
  static diplomat::result<ICU4XPluralRulesWithRanges, ICU4XError> create_cardinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  /**
   * Construct an [`ICU4XPluralRulesWithRanges`] for the given locale, for ordinal numbers
   * 
   * See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_ordinal) for more information.
   */
  static diplomat::result<ICU4XPluralRulesWithRanges, ICU4XError> create_ordinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  /**
   * Get the category for a given number represented as operands
   * 
   * See the [Rust documentation for `category_for`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.category_for) for more information.
   */
  ICU4XPluralCategory category_for(const ICU4XPluralOperands& op) const;

  /**
   * Get all of the categories needed in the current locale
   * 
   * See the [Rust documentation for `categories`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.categories) for more information.
   */
  ICU4XPluralCategories categories() const;

  /**
   * Get the appropriate category for a numeric range.
   * 
   * See the [Rust documentation for `category_for_range`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.category_for_range) for more information.
   */
  ICU4XPluralCategory category_for_range(const ICU4XPluralOperands& start, const ICU4XPluralOperands& end) const;

  /**
   * Get the appropriate category for a numeric range from the categories of its endpoints.
   * 
   * See the [Rust documentation for `resolve_range`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.resolve_range) for more information.
   */
  ICU4XPluralCategory resolve_range(ICU4XPluralCategory start, ICU4XPluralCategory end) const;
  inline const capi::ICU4XPluralRulesWithRanges* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XPluralRulesWithRanges* AsFFIMut() { return this->inner.get(); }
  inline ICU4XPluralRulesWithRanges(capi::ICU4XPluralRulesWithRanges* i) : inner(i) {}
  ICU4XPluralRulesWithRanges() = default;
  ICU4XPluralRulesWithRanges(ICU4XPluralRulesWithRanges&&) noexcept = default;
  ICU4XPluralRulesWithRanges& operator=(ICU4XPluralRulesWithRanges&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XPluralRulesWithRanges, ICU4XPluralRulesWithRangesDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XPluralOperands.hpp"
#include "ICU4XPluralCategories.hpp"

inline diplomat::result<ICU4XPluralRulesWithRanges, ICU4XError> ICU4XPluralRulesWithRanges::create_cardinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto diplomat_result_raw_out_value = capi::ICU4XPluralRulesWithRanges_create_cardinal(provider.AsFFI(), locale.AsFFI());
  diplomat::result<ICU4XPluralRulesWithRanges, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XPluralRulesWithRanges>(ICU4XPluralRulesWithRanges(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XPluralRulesWithRanges, ICU4XError> ICU4XPluralRulesWithRanges::create_ordinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto diplomat_result_raw_out_value = capi::ICU4XPluralRulesWithRanges_create_ordinal(provider.AsFFI(), locale.AsFFI());
  diplomat::result<ICU4XPluralRulesWithRanges, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XPluralRulesWithRanges>(ICU4XPluralRulesWithRanges(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline ICU4XPluralCategory ICU4XPluralRulesWithRanges::category_for(const ICU4XPluralOperands& op) const {
  return static_cast<ICU4XPluralCategory>(capi::ICU4XPluralRulesWithRanges_category_for(this->inner.get(), op.AsFFI()));
}
inline ICU4XPluralCategories ICU4XPluralRulesWithRanges::categories() const {
  capi::ICU4XPluralCategories diplomat_raw_struct_out_value = capi::ICU4XPluralRulesWithRanges_categories(this->inner.get());
  return ICU4XPluralCategories{ .zero = std::move(diplomat_raw_struct_out_value.zero), .one = std::move(diplomat_raw_struct_out_value.one), .two = std::move(diplomat_raw_struct_out_value.two), .few = std::move(diplomat_raw_struct_out_value.few), .many = std::move(diplomat_raw_struct_out_value.many), .other = std::move(diplomat_raw_struct_out_value.other) };
}
inline ICU4XPluralCategory ICU4XPluralRulesWithRanges::category_for_range(const ICU4XPluralOperands& start, const ICU4XPluralOperands& end) const {
  return static_cast<ICU4XPluralCategory>(capi::ICU4XPluralRulesWithRanges_category_for_range(this->inner.get(), start.AsFFI(), end.AsFFI()));
}
inline ICU4XPluralCategory ICU4XPluralRulesWithRanges::resolve_range(ICU4XPluralCategory start, ICU4XPluralCategory end) const {
  return static_cast<ICU4XPluralCategory>(capi::ICU4XPluralRulesWithRanges_resolve_range(this->inner.get(), static_cast<capi::ICU4XPluralCategory>(start), static_cast<capi::ICU4XPluralCategory>(end)));
}
#endif

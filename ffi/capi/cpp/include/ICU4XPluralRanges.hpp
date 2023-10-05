#ifndef ICU4XPluralRanges_HPP
#define ICU4XPluralRanges_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XPluralRanges.h"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XPluralRanges;
#include "ICU4XError.hpp"
#include "ICU4XPluralCategory.hpp"

/**
 * A destruction policy for using ICU4XPluralRanges with std::unique_ptr.
 */
struct ICU4XPluralRangesDeleter {
  void operator()(capi::ICU4XPluralRanges* l) const noexcept {
    capi::ICU4XPluralRanges_destroy(l);
  }
};

/**
 * FFI version of `PluralRanges`.
 * 
 * See the [Rust documentation for `PluralRanges`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRanges.html) for more information.
 */
class ICU4XPluralRanges {
 public:

  /**
   * Construct an [`ICU4XPluralRanges`] for the given locale.
   * 
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRanges.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XPluralRanges, ICU4XError> create(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  /**
   * Get the appropriate category for a numeric range from the categories of its endpoints.
   * 
   * See the [Rust documentation for `category_for_range`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRanges.html#method.category_for_range) for more information.
   */
  ICU4XPluralCategory category_for_range(ICU4XPluralCategory start, ICU4XPluralCategory end) const;
  inline const capi::ICU4XPluralRanges* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XPluralRanges* AsFFIMut() { return this->inner.get(); }
  inline ICU4XPluralRanges(capi::ICU4XPluralRanges* i) : inner(i) {}
  ICU4XPluralRanges() = default;
  ICU4XPluralRanges(ICU4XPluralRanges&&) noexcept = default;
  ICU4XPluralRanges& operator=(ICU4XPluralRanges&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XPluralRanges, ICU4XPluralRangesDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"

inline diplomat::result<ICU4XPluralRanges, ICU4XError> ICU4XPluralRanges::create(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto diplomat_result_raw_out_value = capi::ICU4XPluralRanges_create(provider.AsFFI(), locale.AsFFI());
  diplomat::result<ICU4XPluralRanges, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XPluralRanges>(ICU4XPluralRanges(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline ICU4XPluralCategory ICU4XPluralRanges::category_for_range(ICU4XPluralCategory start, ICU4XPluralCategory end) const {
  return static_cast<ICU4XPluralCategory>(capi::ICU4XPluralRanges_category_for_range(this->inner.get(), static_cast<capi::ICU4XPluralCategory>(start), static_cast<capi::ICU4XPluralCategory>(end)));
}
#endif

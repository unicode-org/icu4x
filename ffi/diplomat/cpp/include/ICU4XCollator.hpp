#ifndef ICU4XCollator_HPP
#define ICU4XCollator_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCollator.h"

class ICU4XDataProvider;
class ICU4XLocale;
struct ICU4XCollatorOptions;
class ICU4XCollator;
#include "ICU4XError.hpp"
#include "ICU4XOrdering.hpp"

/**
 * A destruction policy for using ICU4XCollator with std::unique_ptr.
 */
struct ICU4XCollatorDeleter {
  void operator()(capi::ICU4XCollator* l) const noexcept {
    capi::ICU4XCollator_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `Collator`](https://unicode-org.github.io/icu4x-docs/doc/icu_collator/struct.Collator.html) for more information.
 */
class ICU4XCollator {
 public:
  static diplomat::result<ICU4XCollator, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XCollatorOptions options);

  /**
   * 
   * 
   * See the [Rust documentation for `compare`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html#method.compare) for more information.
   */
  ICU4XOrdering compare(const std::string_view left, const std::string_view right) const;

  /**
   * 
   * 
   * See the [Rust documentation for `compare_utf8`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html#method.compare_utf8) for more information.
   */
  ICU4XOrdering compare_utf8(const diplomat::span<uint8_t> left, const diplomat::span<uint8_t> right) const;

  /**
   * 
   * 
   * See the [Rust documentation for `compare_utf16`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html#method.compare_utf16) for more information.
   */
  ICU4XOrdering compare_utf16(const diplomat::span<uint16_t> left, const diplomat::span<uint16_t> right) const;
  inline const capi::ICU4XCollator* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCollator* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCollator(capi::ICU4XCollator* i) : inner(i) {}
  ICU4XCollator() = default;
  ICU4XCollator(ICU4XCollator&&) noexcept = default;
  ICU4XCollator& operator=(ICU4XCollator&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCollator, ICU4XCollatorDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XCollatorOptions.hpp"

inline diplomat::result<ICU4XCollator, ICU4XError> ICU4XCollator::try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XCollatorOptions options) {
  ICU4XCollatorOptions diplomat_wrapped_struct_options = options;
  auto diplomat_result_raw_out_value = capi::ICU4XCollator_try_new(provider.AsFFI(), locale.AsFFI(), capi::ICU4XCollatorOptions{ .strength = static_cast<capi::ICU4XCollatorStrength>(diplomat_wrapped_struct_options.strength), .alternate_handling = static_cast<capi::ICU4XCollatorAlternateHandling>(diplomat_wrapped_struct_options.alternate_handling), .case_first = static_cast<capi::ICU4XCollatorCaseFirst>(diplomat_wrapped_struct_options.case_first), .max_variable = static_cast<capi::ICU4XCollatorMaxVariable>(diplomat_wrapped_struct_options.max_variable), .case_level = static_cast<capi::ICU4XCollatorCaseLevel>(diplomat_wrapped_struct_options.case_level), .numeric = static_cast<capi::ICU4XCollatorNumeric>(diplomat_wrapped_struct_options.numeric), .backward_second_level = static_cast<capi::ICU4XCollatorBackwardSecondLevel>(diplomat_wrapped_struct_options.backward_second_level) });
  diplomat::result<ICU4XCollator, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCollator>(std::move(ICU4XCollator(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XOrdering ICU4XCollator::compare(const std::string_view left, const std::string_view right) const {
  return static_cast<ICU4XOrdering>(capi::ICU4XCollator_compare(this->inner.get(), left.data(), left.size(), right.data(), right.size()));
}
inline ICU4XOrdering ICU4XCollator::compare_utf8(const diplomat::span<uint8_t> left, const diplomat::span<uint8_t> right) const {
  return static_cast<ICU4XOrdering>(capi::ICU4XCollator_compare_utf8(this->inner.get(), left.data(), left.size(), right.data(), right.size()));
}
inline ICU4XOrdering ICU4XCollator::compare_utf16(const diplomat::span<uint16_t> left, const diplomat::span<uint16_t> right) const {
  return static_cast<ICU4XOrdering>(capi::ICU4XCollator_compare_utf16(this->inner.get(), left.data(), left.size(), right.data(), right.size()));
}
#endif

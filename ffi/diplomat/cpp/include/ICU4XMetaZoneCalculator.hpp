#ifndef ICU4XMetaZoneCalculator_HPP
#define ICU4XMetaZoneCalculator_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XMetaZoneCalculator.h"

class ICU4XDataProvider;
class ICU4XMetaZoneCalculator;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XMetaZoneCalculator with std::unique_ptr.
 */
struct ICU4XMetaZoneCalculatorDeleter {
  void operator()(capi::ICU4XMetaZoneCalculator* l) const noexcept {
    capi::ICU4XMetaZoneCalculator_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html) for more information.
 */
class ICU4XMetaZoneCalculator {
 public:

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.try_new_unstable) for more information.
   */
  static diplomat::result<ICU4XMetaZoneCalculator, ICU4XError> try_new(const ICU4XDataProvider& provider);
  inline const capi::ICU4XMetaZoneCalculator* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XMetaZoneCalculator* AsFFIMut() { return this->inner.get(); }
  inline ICU4XMetaZoneCalculator(capi::ICU4XMetaZoneCalculator* i) : inner(i) {}
  ICU4XMetaZoneCalculator() = default;
  ICU4XMetaZoneCalculator(ICU4XMetaZoneCalculator&&) noexcept = default;
  ICU4XMetaZoneCalculator& operator=(ICU4XMetaZoneCalculator&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XMetaZoneCalculator, ICU4XMetaZoneCalculatorDeleter> inner;
};

#include "ICU4XDataProvider.hpp"

inline diplomat::result<ICU4XMetaZoneCalculator, ICU4XError> ICU4XMetaZoneCalculator::try_new(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XMetaZoneCalculator_try_new(provider.AsFFI());
  diplomat::result<ICU4XMetaZoneCalculator, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XMetaZoneCalculator>(std::move(ICU4XMetaZoneCalculator(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif

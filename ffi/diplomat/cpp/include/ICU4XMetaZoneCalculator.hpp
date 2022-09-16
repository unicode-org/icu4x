#ifndef ICU4XMetazoneCalculator_HPP
#define ICU4XMetazoneCalculator_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XMetazoneCalculator.h"

class ICU4XDataProvider;
class ICU4XMetazoneCalculator;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XMetazoneCalculator with std::unique_ptr.
 */
struct ICU4XMetazoneCalculatorDeleter {
  void operator()(capi::ICU4XMetazoneCalculator* l) const noexcept {
    capi::ICU4XMetazoneCalculator_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `MetazoneCalculator`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneCalculator.html) for more information.
 */
class ICU4XMetazoneCalculator {
 public:

  /**
   * 
   * 
   * See the [Rust documentation for `try_new_unstable`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneCalculator.html#method.try_new_unstable) for more information.
   */
  static diplomat::result<ICU4XMetazoneCalculator, ICU4XError> try_new(const ICU4XDataProvider& provider);
  inline const capi::ICU4XMetazoneCalculator* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XMetazoneCalculator* AsFFIMut() { return this->inner.get(); }
  inline ICU4XMetazoneCalculator(capi::ICU4XMetazoneCalculator* i) : inner(i) {}
  ICU4XMetazoneCalculator() = default;
  ICU4XMetazoneCalculator(ICU4XMetazoneCalculator&&) noexcept = default;
  ICU4XMetazoneCalculator& operator=(ICU4XMetazoneCalculator&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XMetazoneCalculator, ICU4XMetazoneCalculatorDeleter> inner;
};

#include "ICU4XDataProvider.hpp"

inline diplomat::result<ICU4XMetazoneCalculator, ICU4XError> ICU4XMetazoneCalculator::try_new(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XMetazoneCalculator_try_new(provider.AsFFI());
  diplomat::result<ICU4XMetazoneCalculator, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XMetazoneCalculator>(std::move(ICU4XMetazoneCalculator(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif
